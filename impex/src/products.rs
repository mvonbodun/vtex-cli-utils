use futures::{stream, StreamExt, executor::block_on};
use log::*;
use vtex::utils;
use std::num::NonZeroU32;
use std::{error::Error, time::Duration};
use std::fs::File;
use std::sync::Arc;
use governor::{Quota, RateLimiter, Jitter};
use reqwest::{Client, StatusCode};
use vtex::model::Product;
// use crate::csvrecords::ProductLookup;

pub async fn load_products(file_path: String, client: &Client, account_name: String, environment: String, concurrent_requests: usize, rate_limit: NonZeroU32) -> Result<(), Box<dyn Error>> {

    // Read in the category tree and store in a HashMap for lookup
    let categories = utils::get_vtex_category_tree(client, &account_name, &environment).await;
    let category_lookup = utils::parse_category_tree(categories);
    debug!("category_lookup: {:?}", category_lookup.len());

    // Get a lookup for the cateogory name of a category by GroupIdentifier
    let category_identifier_name_lookup = utils::create_category_name_lookup();
    println!("category_identifier_name_lookup: {:?}", category_identifier_name_lookup.len());

    // Get a lookup for the brand_id by brand name
    let brand_id_lookup = utils::create_brand_lookup(client, &account_name, & environment).await;
    debug!("brand_id_lookup: {}", brand_id_lookup.len());
    
    
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/product"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);
    // let out_path = "data/ProductLookup.csv";
    // let mut writer = csv::Writer::from_path(out_path)?;
        
    let mut product_recs: Vec<Product> = Vec::new();

    for line in rdr.deserialize() {
        let mut record: Product = line?;

        // look up the category name
        let parent_cat_name = category_identifier_name_lookup.get(&record.category_unique_identifier).unwrap();
        // Look up the VTEX Category Id
        debug!("PartNumber: {:?}  parent_cat_name: {:?}", &record.ref_id, &parent_cat_name);
        let vtex_cat_id = category_lookup.get(&parent_cat_name.clone()).unwrap();
        record.category_id = Some(*vtex_cat_id);
        // Look up the brand_id
        let brand_id = brand_id_lookup.get(&record.brand_name).unwrap();
        record.brand_id = Some(*brand_id);

        product_recs.push(record);
    }

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));

    let bodies = stream::iter(product_recs)
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
                
                info!("product: {:?}: response: {:?}", record.ref_id, response.status());
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
