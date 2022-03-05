use futures::{stream, StreamExt, executor::block_on};
use governor::{RateLimiter, Quota, Jitter};
use log::*;
use reqwest::{Client, StatusCode};
use vtex::model::{SkuSpecificationAssociation, SkuSpecificationValueAssignment};
use vtex::utils;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;


pub async fn gen_sku_spec_association_file(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    sku_spec_assignment_file: String,
    product_file: String,
    sku_file: String,
) -> Result<(), Box<dyn Error>> {

    // Build a Sku_id lookup fn
    // let sku_id_lookup = utils::create_sku_id_lookup();
    // println!("sku_id_lookup: {}", sku_id_lookup.len());
    // Get a lookup HashMap for the product_ref_id for a sku_ref_id
    let product_ref_id_by_sku_ref_id_lookup = utils::create_sku_product_ref_id_lookup(sku_file);
    debug!("product_ref_id_by_sku_ref_id_lookup: {:?}", product_ref_id_by_sku_ref_id_lookup.len());
    // Get a lookup HashMap for the parent category of a product
    let product_parent_category_lookup = utils::create_product_parent_category_lookup(product_file);
    debug!("product_parent_category_lookkup: {:?}", product_parent_category_lookup.len());
    // Build a category name lookup
    let category_name_lookup = utils::create_category_name_lookup(client, &account_name, &environment).await;
    debug!("category_name_lookup: {}", category_name_lookup.len());
    // Build category id lookup
    let category_id_lookup = utils::create_category_id_lookup(client, &account_name, &environment).await;
    debug!("category_id_lookup: {}", category_id_lookup.len());
    // Build a field id lookup fn get the fields for a category
    let field_id_lookup = utils::create_field_id_lookup(&category_id_lookup, client, &account_name, &environment).await;
    debug!("field_id_lookup: {:?}", field_id_lookup.len());
    // Build a field value id lookup table
    let field_value_id_lookup = utils::create_field_value_id_lookup(&field_id_lookup, client, &account_name, &environment).await;
    debug!("field_value_id_lookup: {:?}", field_value_id_lookup.len());

    // Setup the input and output files
    let in_file = File::open(sku_spec_assignment_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut sku_id_lookup: HashMap<String, i32> = HashMap::new();
    
    let mut x = 0;
    for line in reader.deserialize() {
        let record: SkuSpecificationValueAssignment = line?;

        let sku_id: i32;
        if !sku_id_lookup.contains_key(&record.sku_ref_id) {
            sku_id = utils::get_sku_id_by_ref_id(&record.sku_ref_id, &client, &account_name, &environment).await;
            sku_id_lookup.insert(record.sku_ref_id.clone(), sku_id.clone());
        } else {
            debug!("sku_id_lookup hit. sku_ref_id: {} found.", record.sku_ref_id);
            sku_id = *sku_id_lookup.get(&record.sku_ref_id).unwrap();
        }
        
        // Get the product_ref_id
        let product_ref_id = product_ref_id_by_sku_ref_id_lookup.get(&record.sku_ref_id).unwrap();
        // Get the category identifier for the partnumber
        let parent_category_identifier = product_parent_category_lookup.get(product_ref_id).unwrap();
        // Get category name
        let parent_cat_name = category_name_lookup.get(parent_category_identifier).unwrap();
        // Get the VTEX Category Id
        let vtex_cat_id = category_id_lookup.get(parent_cat_name).unwrap();
        // Build the key to use with field_id_lookup
        let key = vtex_cat_id.to_string().to_owned() + "|" + record.name.as_str();
        // Get the field_id
        let field_id = field_id_lookup.get(&key).expect("failed to find field_id in field_id_lookup");
        // Build the key to use with the field_value_id_lookup
        let field_value_key = field_id.to_string().as_str().to_owned() + "|" + record.value.as_str().trim();
        let field_value_id = field_value_id_lookup.get(&field_value_key).expect("failed to find field_value_id in field_value_id_lookup");
        debug!("record.sku_ref_id {}", &record.sku_ref_id);
        // if sku_id_lookup.contains_key(&record.sku_ref_id) {
            let sku_spec_assign = SkuSpecificationAssociation {
                id: Some(0), // Hardcode to 0, API does not work with None (null)
                sku_id: sku_id.clone(),
                field_id: field_id.clone(),
                field_value_id: Some(field_value_id.clone()),
                text: None,
            };
            writer.serialize(sku_spec_assign)?;
            x = x + 1;
        // }
    }
    // Flush the records
    writer.flush()?;
    println!("records written: {}", x);
    
    Ok(())
}

pub async fn load_sku_specs(file_path: String, client: &Client, account_name: String, environment: String, concurrent_requests: usize, rate_limit: NonZeroU32) -> Result<(), Box<dyn Error>> {

    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/stockkeepingunit/{skuId}/specification"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut skuspecassoc_rec: Vec<SkuSpecificationAssociation> = Vec::new();

    for line in rdr.deserialize() {
        let record: SkuSpecificationAssociation = line?;
        debug!("SkuSpecificationAssociation Record: {:?}", record);
        skuspecassoc_rec.push(record);
    }

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));

    let bodies = stream::iter(skuspecassoc_rec)
        .map(|record| {
            let client = &client;
            let url = &url;
            let lim = Arc::clone(&lim);
            async move {
                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));

                let url = url.replace("{skuId}", record.sku_id.to_string().as_str());

                let response = client
                    .post(url)
                    .json(&record)
                    .send()
                    .await?;

                    let status = response.status();
                    info!("product: {:?}  text: {:?}:  response: {:?}", record.sku_id, record.text, status);
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
    
    info!("finished load_sku_spec_associations");

    Ok(())
}