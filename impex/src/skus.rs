use futures::{stream, StreamExt, executor::block_on};
use governor::{Quota, RateLimiter, Jitter};
use log::*;
use reqwest::{Client, StatusCode};
use vtex::{model::Sku, utils};

use std::{error::Error, fs::File, time::Duration, sync::Arc, num::NonZeroU32, collections::HashMap};

pub async fn load_skus(file_path: String, client: &Client, account_name: String, environment: String, concurrent_requests: usize, rate_limit: NonZeroU32) -> Result<(), Box<dyn Error>> {

    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/stockkeepingunit"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);
    // let out_path = "data/SkuLookup.csv";
    // let mut writer = csv::Writer::from_path(out_path)?;

    let mut sku_recs: Vec<Sku> = Vec::new(); 
    let mut product_lookup: HashMap<String, i32> = HashMap::new();

    for line in rdr.deserialize() {
        let mut record: Sku = line?;
        debug!("sku_record: {:?}", record);
        let product_id: i32;
        if !product_lookup.contains_key(&record.product_ref_id) {
            product_id = utils::get_product_by_ref_id(&record.product_ref_id, &client, &account_name, &environment).await;
            product_lookup.insert(record.product_ref_id.clone(), product_id.clone());
            record.product_id = Some(product_id);
        } else {
            debug!("product_lookup hit. product_ref_id: {} found.", record.product_ref_id);
            record.product_id = Some(*product_lookup.get(&record.product_ref_id).unwrap());
        }
        sku_recs.push(record);
    }
    debug!("sku_recs length: {}", sku_recs.len());

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));

    let bodies = stream::iter(sku_recs)
        .map(|record| {
            let client = &client;
            let url = &url;
            let lim = Arc::clone(&lim);
            async move {
                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));

                let response = client
                    .post(url)
                    .json(&record)
                    .send()
                    .await?;
                
                info!("sku: {:?}: response: {:?}", record.ref_id, response.status());
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
    
    info!("finished load_skus");

    Ok(())
}

// pub async fn load_skus(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {

//     let input = File::open(file_path)?;
//     let mut rdr = csv::Reader::from_reader(input);
//     let out_path = "data/SkuLookup.csv";
//     let mut writer = csv::Writer::from_path(out_path)?;

//     for line in rdr.deserialize() {
//         let record: Sku = line?;
//         // println!("SKU: {:?}", record);

//         let response = client
//             .post(&url)
//             .json(&record)
//             .send()
//             .await?;

//         println!("response.status: {}", response.status());
//         match response.status() {
//             StatusCode::OK => {
//                 let sku_json = response.text().await?;
//                 // println!("sku_json: {:?}", sku_json);
//                 let result: Result<Sku, serde_json::Error> = serde_json::from_str(&sku_json);
//                 match result {
//                     Ok(sku) => {
//                         println!("sku_id: {}", sku.id.unwrap());
//                         let sku_lookup = SkuLookup {
//                             part_number: sku.ref_id.unwrap(),
//                             sku_id: sku.id.unwrap(),
//                         };
//                         writer.serialize(&sku_lookup)?;
//                     },
//                     Err(e) => println!("deserialize product error: {:?}", e),
//                 }
//             },
//             _ => {
//                 println!("Status Code: [{:?}] Error: [{:#?}] \n record: {:?}", response.status(), response.text().await?, record);
//             },
//         }
//     }
//     // Flush the writer
//     writer.flush()?;

//     Ok(())
// }