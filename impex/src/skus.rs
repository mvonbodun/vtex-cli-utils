use futures::{executor::block_on, stream, StreamExt};
use governor::{Jitter, Quota, RateLimiter};
use log::*;
use reqwest::{Client, StatusCode};
use vtex::{model::Sku, utils};

use std::{
    collections::HashMap, error::Error, fs::File, num::NonZeroU32, sync::Arc, time::Duration,
};

pub async fn load_skus(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    rate_limit: NonZeroU32,
) -> Result<(), Box<dyn Error>> {
    info!("Starting SKU load");
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/stockkeepingunit"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut sku_recs: Vec<Sku> = Vec::new();
    let mut product_lookup: HashMap<String, i32> = HashMap::new();

    info!("Starting Product Id lookup");
    for line in rdr.deserialize() {
        let mut record: Sku = line?;
        debug!("sku_record: {:?}", record);
        let product_id: i32;
        if !product_lookup.contains_key(&record.product_ref_id) {
            product_id = utils::get_product_by_ref_id(
                &record.product_ref_id,
                &client,
                &account_name,
                &environment,
            )
            .await;
            product_lookup.insert(record.product_ref_id.clone(), product_id.clone());
            record.product_id = Some(product_id);
        } else {
            debug!(
                "product_lookup hit. product_ref_id: {} found.",
                record.product_ref_id
            );
            record.product_id = Some(*product_lookup.get(&record.product_ref_id).unwrap());
        }
        sku_recs.push(record);
    }
    info!("Finished Product Id lookup");
    debug!("sku_recs length: {}", sku_recs.len());

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));

    let bodies = stream::iter(sku_recs)
        .map(|record| {
            let client = &client;
            let url = &url;
            let lim = Arc::clone(&lim);
            async move {
                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));

                let response = client.post(url).json(&record).send().await?;

                info!(
                    "sku: {:?}: response: {:?}",
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
                Ok(_b) => (),
                Err(e) => error!("error: {:?}", e),
            }
        })
        .await;

    info!("finished SKU load");

    Ok(())
}
