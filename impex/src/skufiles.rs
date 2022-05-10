use futures::{executor::block_on, stream, StreamExt};
use governor::{Jitter, Quota, RateLimiter};
use log::*;
use regex::Regex;
use reqwest::{Client, StatusCode};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashSet, error::Error, fs::File};
use vtex::model::{Sku, SkuFile};
use vtex::utils;

pub async fn gen_sku_file(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    sku_file: String,
) -> Result<(), Box<dyn Error>> {
    info!("Starting generation of SKU Files file");

    // Parse the skufile and verify it deserializes the records
    info!("Start: Reading input file to ensure values can be parsed");
    // Setup the input and output files
    let in_file = File::open(sku_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;
    let mut sku_recs: Vec<Sku> = Vec::new();
    let mut e = 0;
    for line in reader.deserialize() {
        match line {
            Ok(record) => {
                let sku_rec: Sku = record;
                sku_recs.push(sku_rec);
            }
            Err(err) => {
                error!("Error parsing row: {:?}", err);
                e += 1;
            }
        }
    }
    info!("Finished: Reading input file");
    info!(
        "Records successfully read: {}. Records not read (errors): {}",
        sku_recs.len(),
        e
    );
    // Build a Sku_id lookup fn
    // let sku_id_lookup = utils::create_sku_id_lookup(client, &account_name, &environment).await;

    // Create HashSet to track if this is the first time the part_number appears
    let mut part_number_set: HashSet<String> = HashSet::new();
    // Regex to make the name friendly
    let re = Regex::new(r"([^\w\s-])").unwrap();
    debug!("Begin reading Sku input file");
    let mut x = 0;
    for line in sku_recs {
        let record: Sku = line;
        debug!("sku record: {:?}", record);
        // get the sku_id
        let get_sku_id =
            utils::get_sku_id_by_ref_id(&record.ref_id, client, &account_name, &environment).await;
        match get_sku_id {
            Ok(sku_id) => {
                let is_main: bool;
                if part_number_set.contains(&record.product_ref_id) {
                    is_main = false;
                } else {
                    is_main = true;
                }
                // Remove special characters from the name
                let name = record.name.replace(" ", "-");
                debug!("name: {}", name);
                let name = re.replace_all(&name, "");
                debug!("after regex pattern replacement: {}", name);
                // Determine if there is more than one image for the SKU
                let img_url = record.image_url.expect("missing Image Url for SKU");
                let semicolon: char = ';';
                let iter = img_url.split(semicolon);
                let mut y = 0;
                for i in iter {
                    y += 1;
                    let mut name_with_number = name.to_string();
                    name_with_number.push('_');
                    name_with_number.push_str(&y.to_string());
                    let sku_file = SkuFile {
                        id: None,
                        sku_id,
                        is_main: Some(is_main),
                        archive_id: None,
                        name: Some(name_with_number.to_string()),
                        label: Some(name_with_number.to_string()),
                        url: Some(i.to_string()),
                    };
                    writer.serialize(sku_file)?;
                    part_number_set.insert(record.product_ref_id.clone());
                    x += 1;
                }
            }
            Err(err) => {
                error!("error occured getting sku_id: {:?}", err);
                ()
            }
        }
    }
    // Flush the records
    writer.flush()?;
    info!("records writtern: {}", x);
    info!("Finished generating SKU Files file");

    Ok(())
}

pub async fn load_sku_files(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    rate_limit: NonZeroU32,
) -> Result<(), Box<dyn Error>> {
    info!("Starting load of SKU Files file");
    let url =
        "https://{accountName}.{environment}.com.br/api/catalog/pvt/stockkeepingunit/{skuId}/file"
            .replace("{accountName}", &account_name)
            .replace("{environment}", &environment);

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut skufile_rec: Vec<SkuFile> = Vec::new();

    for line in rdr.deserialize() {
        let record: SkuFile = line?;
        debug!("SkuFile Record: {:?}", record);
        skufile_rec.push(record);
    }

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));

    let bodies = stream::iter(skufile_rec)
        .map(|record| {
            let client = &client;
            let url = &url;
            let lim = Arc::clone(&lim);
            async move {
                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));

                let url = url.replace("{skuId}", record.sku_id.to_string().as_str());

                let response = client.post(url).json(&record).send().await?;

                let status = response.status();
                info!(
                    "sku_id: {:?}  image: {:?}:  response: {:?}",
                    record.sku_id, record.url, status
                );
                let text = response.text().await;
                if status != StatusCode::OK {
                    info!("text: {:?}", text);
                }
                text
            }
        })
        .buffer_unordered(concurrent_requests);
    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => info!("output: {:?}", b),
                Err(e) => error!("error: {:?}", e),
            }
        })
        .await;

    info!("finished loading SKU Files file");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_split() {
        let urls = "https://www.google.com";
        let iter = urls.split(";");
        println!("count: {}", iter.count());
    }
}
