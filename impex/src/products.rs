use futures::{executor::block_on, stream, StreamExt};
use governor::{Jitter, Quota, RateLimiter};
use log::*;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;
use std::fs::File;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::{error::Error, time::Duration};
use vtex::model::Product;
use vtex::utils;

// use crate::categories;

pub async fn load_products(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    rate_limit: NonZeroU32,
    skip_cat_lookup: usize
) -> Result<(), Box<dyn Error>> {
    info!("Starting load of products");
    // Read in the category tree and store in a HashMap for lookup
    let mut categories = Vec::new();
    let mut category_lookup: HashMap<String, i32> = HashMap::new();
    let mut category_identifier_name_lookup: HashMap<String, String> = HashMap::new();
    
    if skip_cat_lookup == 0 {
        categories = utils::get_vtex_category_tree(client, &account_name, &environment).await;
        category_lookup = utils::parse_category_tree(categories);
        debug!("category_lookup: {:?}", category_lookup.len());
    
        // Get a lookup for the cateogory name of a category by GroupIdentifier
        category_identifier_name_lookup =
            utils::create_category_name_lookup(client, &account_name, &environment).await;
        debug!(
            "category_identifier_name_lookup: {:?}",
            category_identifier_name_lookup.len()
        );
    }

    // Get a lookup for the brand_id by brand name
    let brand_id_lookup = utils::create_brand_lookup(client, &account_name, &environment).await;
    debug!("brand_id_lookup: {}", brand_id_lookup.len());

    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/product"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut product_recs: Vec<Product> = Vec::new();

    for line in rdr.deserialize() {
        let mut record: Product = line?;

        if skip_cat_lookup == 0 {
            // look up the category name
            let cat_unique_identifier = record.category_unique_identifier.as_ref().unwrap();
            let parent_cat_name = category_identifier_name_lookup
                .get(cat_unique_identifier)
                .unwrap();
            // Look up the VTEX Category Id
            debug!(
                "ref_id: {:?}  parent_cat_name: {:?}",
                &record.ref_id, &parent_cat_name
            );
            let vtex_cat_id = category_lookup.get(&parent_cat_name.clone()).unwrap();
            record.category_id = Some(*vtex_cat_id);
        }
        // Look up the brand_id
        let brand_name = record
            .brand_name
            .as_ref()
            .unwrap_or_else(|| panic!("BrandName missing in CSV for SKU Ref: {:?}", record.ref_id));
        let brand_id = brand_id_lookup.get(brand_name).unwrap_or_else(|| panic!("Brand Name: {:?} not found in lookup table.  Make sure Brand Name in the BrandName column in Products.csv matches Brand Name in the Name column of the Brands.csv file.  The values are case sensitive.", record.brand_name));
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

                let response = client.post(url).json(&record).send().await?;

                info!(
                    "product: {:?}: response: {:?}",
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
                Ok(b) => info!("output: {:?}", b),
                Err(e) => error!("error: {:?}", e),
            }
        })
        .await;

    info!("finished loading products");

    Ok(())
}

pub async fn update_products(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    rate_limit: NonZeroU32,
    skip_cat_lookup: usize
) -> Result<(), Box<dyn Error>> {
    info!("Starting load of products");
    // Read in the category tree and store in a HashMap for lookup
    let mut categories = Vec::new();
    let mut category_lookup: HashMap<String, i32> = HashMap::new();
    let mut category_identifier_name_lookup: HashMap<String, String> = HashMap::new();
    
    debug!("skip_cat_lookup={}", skip_cat_lookup);
    if skip_cat_lookup == 0 {
        categories = utils::get_vtex_category_tree(client, &account_name, &environment).await;
        category_lookup = utils::parse_category_tree(categories);
        debug!("category_lookup: {:?}", category_lookup.len());
    
        // Get a lookup for the cateogory name of a category by GroupIdentifier
        category_identifier_name_lookup =
            utils::create_category_name_lookup(client, &account_name, &environment).await;
        debug!(
            "category_identifier_name_lookup: {:?}",
            category_identifier_name_lookup.len()
        );
    }

    // Get a lookup for the brand_id by brand name
    let brand_id_lookup = utils::create_brand_lookup(client, &account_name, &environment).await;
    debug!("brand_id_lookup: {}", brand_id_lookup.len());

    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/product/{productId}"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut product_recs: Vec<Product> = Vec::new();

    for line in rdr.deserialize() {
        let mut record: Product = line?;

        if skip_cat_lookup == 0 {
            // look up the category name
            let cat_unique_identifier = record.category_unique_identifier.as_ref().unwrap();
            let parent_cat_name = category_identifier_name_lookup
                .get(cat_unique_identifier)
                .unwrap();
            // Look up the VTEX Category Id
            debug!(
                "ref_id: {:?}  parent_cat_name: {:?}",
                &record.ref_id, &parent_cat_name
            );
            let vtex_cat_id = category_lookup.get(&parent_cat_name.clone()).unwrap();
            record.category_id = Some(*vtex_cat_id);
        }
        // Look up the brand_id
        let brand_name = record
            .brand_name
            .as_ref()
            .unwrap_or_else(|| panic!("BrandName missing in CSV for SKU Ref: {:?}", record.ref_id));
        let brand_id = brand_id_lookup.get(brand_name).unwrap_or_else(|| panic!("Brand Name: {:?} not found in lookup table.  Make sure Brand Name in the BrandName column in Products.csv matches Brand Name in the Name column of the Brands.csv file.  The values are case sensitive.", record.brand_name));
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
                let url_with_product_id =
                    url.replace("{productId}", record.id.unwrap().to_string().as_str());

                let response = client.put(url_with_product_id).json(&record).send().await?;

                info!(
                    "product: {:?}: response: {:?}",
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
                Ok(b) => info!("output: {:?}", b),
                Err(e) => error!("error: {:?}", e),
            }
        })
        .await;

    info!("finished loading products");

    Ok(())
}
