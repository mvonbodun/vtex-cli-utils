use log::*;
use std::error::Error;
use std::fs::File;
use reqwest::{Client};
use vtex::model::Inventory;
use futures::{stream, StreamExt };

pub async fn load_inventory(file_path: String, client: &Client, account_name: String, environment: String, concurrent_requests: usize) -> Result<(), Box<dyn Error>> {

    let url = "https://{accountName}.{environment}.com.br/api/logistics/pvt/inventory/skus/{skuId}/warehouses/{warehouseId}"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut inv_recs: Vec<Inventory> = Vec::new();

    for line in rdr.deserialize() {
        let record: Inventory = line?;
        inv_recs.push(record);
    }
    info!("inventory records: {:?}", inv_recs.len());

    let bodies = stream::iter(inv_recs)
        .map(|record| {
            let client = &client;
            let url = &url;
            async move {
                let url_replaced = url
                .replace("{skuId}", record.sku_id.to_string().as_str())
                .replace("{warehouseId}", record.warehouse_id.to_string().as_str());

                let response = client
                .put(url_replaced)
                .json(&record)
                .send()
                .await?;

                info!("sku: {:?}: response: {:?}", record.sku_id, response.status());
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
    
    info!("finished load_inventory");

    Ok(())    
    
}

#[cfg(test)]
mod tests {

    use futures::executor::block_on;
    use futures::{stream, StreamExt};
    use governor::{prelude::*, Quota, RateLimiter};
    use nonzero_ext::*;
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    #[test]
    fn stream() {
        let lim = Arc::new(RateLimiter::direct(Quota::per_second(nonzero!(10u32))));
        let mut stream = stream::repeat(()).ratelimit_stream(&lim);
        let i = Instant::now();

        for _ in 0..10 {
            block_on(stream.next());
        }
  
        assert!(i.elapsed() <= Duration::from_millis(100));

        block_on(stream.next());
        assert!(i.elapsed() > Duration::from_millis(100));
        assert!(i.elapsed() <= Duration::from_millis(200));

        block_on(stream.next());
        assert!(i.elapsed() > Duration::from_millis(200));
        assert!(i.elapsed() <= Duration::from_millis(300));
    }
}