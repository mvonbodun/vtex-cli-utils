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
    // Build a Sku_id lookup fn
    let sku_id_lookup = utils::create_sku_id_lookup(client, &account_name, &environment).await;

    // Setup the input and output files
    let in_file = File::open(sku_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    debug!("Begin reading Sku input file");
    let mut x = 0;
    for line in reader.deserialize() {
        let record: Sku = line?;
        debug!("sku record: {:?}", record);

        // Only create a record if the EAN is populated
        if record.ean.is_some() {
            let sku_ean = SkuEan {
                sku_id: *sku_id_lookup.get(&record.ref_id).unwrap(),
                ean: record.ean.unwrap().clone(),
            };
            writer.serialize(sku_ean)?;
            x += 1;
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

    info!("finished loading SKU EAN file");

    Ok(())
}
