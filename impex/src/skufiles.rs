use futures::{stream, StreamExt, executor::block_on};
use governor::{RateLimiter, Quota, Jitter};
use log::*;
use regex::Regex;
use reqwest::{Client, StatusCode};
use vtex::model::{SkuFile, Sku};
use vtex::utils;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, fs::File, collections::HashSet};

pub async fn gen_sku_file(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    sku_file: String
) -> Result<(), Box<dyn Error>> {

    // Build a Sku_id lookup fn
    let sku_id_lookup = utils::create_sku_id_lookup(client, &account_name, &environment).await;

    // Setup the input and output files
    let in_file = File::open(sku_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    // Create HashSet to track if this is the first time the part_number appears
    let mut part_number_set: HashSet<String> = HashSet::new();
    // Regex to make the name friendly
    let re = Regex::new(r"([^\w\s-])").unwrap();
    debug!("Begin reading Sku input file");
    let mut x = 0;
    for line in reader.deserialize() {
        let record: Sku = line?;
        debug!("sku record: {:?}", record);
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
        if sku_id_lookup.contains_key(&record.ref_id) {
            let sku_file = SkuFile {
                id: None,
                sku_id: *sku_id_lookup.get(&record.ref_id).unwrap(),
                is_main: Some(is_main),
                archive_id: None,
                name: Some(name.to_string()),
                label: Some(name.to_string()),
                url: record.image_url,
            };
            writer.serialize(sku_file)?;
            part_number_set.insert(record.product_ref_id);
            x = x + 1;
        }
    
    }
    // Flush the records
    writer.flush()?;
    println!("records writtern: {}", x);

    Ok(())
}

pub async fn load_sku_files(file_path: String, client: &Client, account_name: String, environment: String, concurrent_requests: usize, rate_limit: NonZeroU32) -> Result<(), Box<dyn Error>> {

    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/stockkeepingunit/{skuId}/file"
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

                let response = client
                    .post(url)
                    .json(&record)
                    .send()
                    .await?;

                    let status = response.status();
                    info!("sku_id: {:?}  image: {:?}:  response: {:?}", record.sku_id, record.url, status);
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
                Ok(_b) => (),
                Err(e) => error!("error: {:?}", e),
            }
        })
        .await;
    
    info!("finished load_sku_files");

    Ok(())
}
