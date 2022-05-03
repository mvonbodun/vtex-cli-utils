// use governor::{Quota, RateLimiter, Jitter};
use log::*;
use std::fs::File;
use std::{error::Error, num::NonZeroU32};
// use std::sync::Arc;
// use std::time::Duration;
use futures::{stream, StreamExt};
use reqwest::Client;
use vtex::model::Inventory;
use vtex::utils;
// use futures::executor::block_on;

pub async fn load_inventory(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    _rate_limit: NonZeroU32,
) -> Result<(), Box<dyn Error>> {
    info!("Starting load of Inventory");
    let url = "https://{accountName}.{environment}.com.br/api/logistics/pvt/inventory/skus/{skuId}/warehouses/{warehouseId}"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut inv_recs: Vec<Inventory> = Vec::new();

    // Build a sku_id lookup
    let sku_id_lookup = utils::create_sku_id_lookup(client, &account_name, &environment).await;

    for line in rdr.deserialize() {
        let mut record: Inventory = line?;
        let sku_id = *sku_id_lookup.get(&record.ref_id).unwrap();
        record.sku_id = Some(sku_id);
        inv_recs.push(record);
    }
    info!("inventory records: {:?}", inv_recs.len());

    //    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));

    let bodies = stream::iter(inv_recs)
        .map(|record| {
            let client = &client;
            let url = &url;
            //             let lim = Arc::clone(&lim);
            async move {
                //                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(10))));

                let url_replaced = url
                    .replace("{skuId}", record.sku_id.unwrap().to_string().as_str())
                    .replace("{warehouseId}", record.warehouse_id.to_string().as_str());

                let response = client.put(url_replaced).json(&record).send().await?;

                info!(
                    "sku: {:?}: response: {:?}",
                    record.sku_id,
                    response.status()
                );
                response.text().await
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

    info!("finished loading inventory");

    Ok(())
}

// #[cfg(test)]
// mod tests {

//     use futures::executor::block_on;
//     use futures::{stream, StreamExt};
//     use governor::{prelude::*, Quota, RateLimiter};
//     use nonzero_ext::*;
//     use std::sync::Arc;
//     use std::time::{Duration, Instant};

//     #[test]
//     fn stream() {
//         let lim = Arc::new(RateLimiter::direct(Quota::per_second(nonzero!(10u32))));
//         let mut stream = stream::repeat(()).ratelimit_stream(&lim);
//         let i = Instant::now();

//         for _ in 0..10 {
//             block_on(stream.next());
//         }

//         assert!(i.elapsed() <= Duration::from_millis(100));

//         block_on(stream.next());
//         assert!(i.elapsed() > Duration::from_millis(100));
//         assert!(i.elapsed() <= Duration::from_millis(200));

//         block_on(stream.next());
//         assert!(i.elapsed() > Duration::from_millis(200));
//         assert!(i.elapsed() <= Duration::from_millis(300));
//     }
// }
