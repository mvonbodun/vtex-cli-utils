use futures::{executor::block_on, stream, StreamExt};
use governor::{Jitter, Quota, RateLimiter};
use log::*;
use reqwest::{Client, StatusCode};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, fs::File};
use vtex::model::{Sku, SkuEan};
use vtex::utils;

pub async fn gen_sku_ean_file(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    sku_file: String,
) -> Result<(), Box<dyn Error>> {
    info!("Starting generation of SKU EAN file");

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
                // Only add records that have an EAN
                let sku_rec: Sku = record;
                if sku_rec.ean.is_some() {
                    sku_recs.push(sku_rec);
                }
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
    //let sku_id_lookup = utils::create_sku_id_lookup(client, &account_name, &environment).await;

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
                let sku_ean = SkuEan {
                    sku_id,
                    ean: record.ean.unwrap().clone(),
                };
                writer.serialize(sku_ean)?;
                x += 1;
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
    info!("Finished generating SKU EAN file");

    Ok(())
}

pub async fn load_sku_eans(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    rate_limit: NonZeroU32,
) -> Result<(), Box<dyn Error>> {
    info!("Starting load of SKU EAN file");
    let url =
        "https://{accountName}.{environment}.com.br/api/catalog/pvt/stockkeepingunit/{skuId}/ean/{ean}"
            .replace("{accountName}", &account_name)
            .replace("{environment}", &environment);

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut skuean_rec: Vec<SkuEan> = Vec::new();

    for line in rdr.deserialize() {
        let record: SkuEan = line?;
        debug!("SkuEan Record: {:?}", record);
        skuean_rec.push(record);
    }

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));

    let bodies = stream::iter(skuean_rec)
        .map(|record| {
            let client = &client;
            let url = &url;
            let lim = Arc::clone(&lim);
            async move {
                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));

                let url = url.replace("{skuId}", record.sku_id.to_string().as_str());
                let url = url.replace("{ean}", record.ean.as_str());

                let response = client.post(url).json(&record).send().await?;

                let status = response.status();
                info!(
                    "sku_id: {:?}  ean: {:?}:  response: {:?}",
                    record.sku_id, record.ean, status
                );
                let text = response.text().await;
                if status != StatusCode::OK {
                    error!("text: {:?}", text);
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

    info!("finished loading SKU EAN file");

    Ok(())
}
