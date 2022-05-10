use futures::{executor::block_on, stream, StreamExt};
use governor::{Jitter, Quota, RateLimiter};
use log::*;
use reqwest::{Client, StatusCode};
use std::fs::File;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::{error::Error, time::Duration};
use vtex::model::Price;
use vtex::utils;

pub async fn load_prices(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    rate_limit: NonZeroU32,
) -> Result<(), Box<dyn Error>> {
    info!("Starting Price load");
    let url = "https://api.vtex.com/{accountName}/pricing/prices/{skuId}"
        .replace("{accountName}", &account_name);
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut price_recs: Vec<Price> = Vec::new();
    info!("Start: Reading input file to ensure values can be parsed");
    let mut e = 0;
    for line in rdr.deserialize() {
        match line {
            Ok(record) => {
                let price_rec: Price = record;
                price_recs.push(price_rec);
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
        price_recs.len(),
        e
    );

    // After full file read and removing non-deserialized records
    let mut price_recs_with_skuid: Vec<Price> = Vec::new();
    for mut line in price_recs {
        debug!("line in price_recs: {:?}", line);
        let get_sku_id =
            utils::get_sku_id_by_ref_id(&line.ref_id, client, &account_name, &environment).await;
        match get_sku_id {
            Ok(sku_id) => {
                line.sku_id = Some(sku_id);
                price_recs_with_skuid.push(line);
            }
            Err(err) => {
                error!("Error: price record will be skipped: {}", err);
            }
        }
    }
    debug!(
        "price_recs_with_skuid length: {}",
        price_recs_with_skuid.len()
    );

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));
    // let mut bodies = stream::iter(price_recs).ratelimit_stream(&lim);
    let bodies = stream::iter(price_recs_with_skuid)
        .map(|record| {
            let client = &client;
            let url = &url;
            let lim = Arc::clone(&lim);
            async move {
                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));
                let url_with_sku_id =
                    url.replace("{skuId}", record.sku_id.unwrap().to_string().as_str());

                let response = client.put(&url_with_sku_id).json(&record).send().await?;

                info!(
                    "sku: {:?} ref_id: {:?} response: {:?}",
                    record.sku_id,
                    record.ref_id,
                    response.status()
                );
                if response.status() == StatusCode::TOO_MANY_REQUESTS {
                    info!("headers: {:?}", response.headers());
                }
                response.text().await
            }
        })
        .buffer_unordered(concurrent_requests);
    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => debug!("output: {:?}", b),
                Err(e) => error!("error: {:?}", e),
            }
        })
        .await;

    info!("finished price load");

    Ok(())
}
