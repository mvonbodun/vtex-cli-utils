use governor::{Quota, RateLimiter, Jitter};
use log::*;
use reqwest::{Client, StatusCode};
use vtex::model::Price;
use vtex::utils;
use std::num::NonZeroU32;
use std::{error::Error, time::Duration};
use std::fs::File;
use std::sync::Arc;
use futures::{stream, StreamExt, executor::block_on};

pub async fn load_prices(file_path: String, client: &Client, account_name: String, environment: String, concurrent_requests: usize, rate_limit: NonZeroU32) -> Result<(), Box<dyn Error>> {

    let url = "https://api.vtex.com/{accountName}/pricing/prices/{skuId}"
        .replace("{accountName}", &account_name);
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut price_recs: Vec<Price> = Vec::new();

    // Build a sku_id lookup
    let sku_id_lookup = utils::create_sku_id_lookup(client, &account_name, &environment).await;

    for line in rdr.deserialize() {
        let mut record: Price = line?;
        let sku_id = sku_id_lookup.get(&record.ref_id).unwrap().clone();
        record.sku_id = Some(sku_id);
        price_recs.push(record);
    }

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));
    // let mut bodies = stream::iter(price_recs).ratelimit_stream(&lim);
    let bodies = stream::iter(price_recs)
        .map(|record| {
            let client = &client;
            let url = &url;
            let lim = Arc::clone(&lim);
            async move {
                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));
                let url_with_sku_id = url.replace("{skuId}", record.sku_id.unwrap().to_string().as_str());

                let response = client
                    .put(&url_with_sku_id)
                    .json(&record)
                    .send()
                    .await?;
                
                info!("sku: {:?}: response: {:?}", record.sku_id, response.status());
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
                Ok(_b) => (),
                Err(e) => error!("error: {:?}", e),
            }
        })
        .await;
    
    info!("finished load_prices");

    Ok(())

}