use futures::{executor::block_on, stream, StreamExt};
use governor::{Jitter, Quota, RateLimiter};
use log::*;
use reqwest::{Client, StatusCode};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use vtex::csvrecords::SkuSpecificationAssignmentAlternate;
use vtex::model::{SkuSpecificationAssociation, SkuSpecificationValueAssignment};
use vtex::utils;

pub async fn gen_sku_spec_assign_file_alternate(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    sku_spec_assignment_file: String,
    product_file: String,
    sku_file: String,
) -> Result<(), Box<dyn Error>> {
    info!("Staring generation of SKU Spec Association file");

    // Setup the input and output files
    let in_file = File::open(sku_spec_assignment_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut sku_specs: Vec<SkuSpecificationAssignmentAlternate> = Vec::new();
    let mut ref_ids: Vec<String> = Vec::new();
    let mut e = 0;
    for line in reader.deserialize() {
        match line {
            Ok(record) => {
                let sku_spec: SkuSpecificationAssignmentAlternate = record;
                let ref_id = sku_spec.sku_ref_id.clone();
                sku_specs.push(sku_spec);
                ref_ids.push(ref_id);
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
        sku_specs.len(),
        e
    );
    debug!("ref_ids.len(): {}", ref_ids.len());

    // Get a lookup HashMap for the product_ref_id for a sku_ref_id
    let product_ref_id_by_sku_ref_id_lookup = utils::create_sku_product_ref_id_lookup(sku_file);
    debug!(
        "product_ref_id_by_sku_ref_id_lookup: {:?}",
        product_ref_id_by_sku_ref_id_lookup.len()
    );
    // Write out the data to validate
    debug!("ProductId,SkuRefId");
    for (k, v) in product_ref_id_by_sku_ref_id_lookup.clone() {
        debug!("{},{}", v, k);
    }

    // Get a lookup HashMap for the parent category of a product
    let product_parent_category_lookup =
        utils::create_product_parent_category_lookup(&product_file);
    debug!(
        "product_parent_category_lookkup: {:?}",
        product_parent_category_lookup.len()
    );
    info!("Staring generation of SKU Spec Association file");
    // Build a Sku_id lookup fn
    // let sku_id_lookup = utils::create_sku_id_lookup(client, &account_name, &environment).await;
    let sku_id_lookup =
        utils::get_sku_ids_by_ref_ids(ref_ids, client, &account_name, &environment).await;
    debug!("sku_id_lookup: {}", sku_id_lookup.len());

    // Write header record
    writer.write_record(&[
        "ProductRefId",
        "Name",
        "Position",
        "AllowedValue1",
        "AllowedValue2",
        "AllowedValue3",
        "AllowedValue4",
        "AllowedValue5",
        "AllowedValue6",
        "AllowedValue7",
        "AllowedValue8",
        "AllowedValue9",
        "AllowedValue10",
        "AllowedValue11",
        "AllowedValue12",
        "AllowedValue13",
        "AllowedValue14",
        "AllowedValue15",
        "AllowedValue16",
        "AllowedValue17",
        "AllowedValue18",
        "AllowedValue19",
        "AllowedValue20",
        "AllowedValue21",
        "AllowedValue22",
        "AllowedValue23",
        "AllowedValue24",
        "AllowedValue25",
        "AllowedValue26",
        "AllowedValue27",
        "AllowedValue28",
        "AllowedValue29",
        "AllowedValue30",
        "AllowedValue31",
        "AllowedValue32",
        "AllowedValue33",
        "AllowedValue34",
        "AllowedValue35",
        "AllowedValue36",
        "AllowedValue37",
        "AllowedValue38",
        "AllowedValue39",
        "AllowedValue40",
        "AllowedValue41",
        "AllowedValue42",
        "AllowedValue43",
        "AllowedValue44",
        "AllowedValue45",
        "AllowedValue46",
        "AllowedValue47",
        "AllowedValue48",
        "AllowedValue49",
        "AllowedValue50",
        "AllowedValue51",
        "AllowedValue52",
        "AllowedValue53",
        "AllowedValue54",
        "AllowedValue55",
        "AllowedValue56",
        "AllowedValue57",
        "AllowedValue58",
        "AllowedValue59",
        "AllowedValue60",
    ])?;

    let mut product_allowed_values_map_color: HashMap<i32, Vec<String>> = HashMap::new();
    let mut product_allowed_values_map_size: HashMap<i32, Vec<String>> = HashMap::new();
    for line in sku_specs {
        // get the product_id for the sku_ref_id
        let get_product_id = product_ref_id_by_sku_ref_id_lookup.get(&line.sku_ref_id);
        if get_product_id.is_some() {
            let product_id = get_product_id.unwrap().parse::<i32>().unwrap();
            // Test to see if the record exists
            if product_allowed_values_map_color.contains_key(&product_id) {
                if line.color.is_some() {
                    let allowed_values_map = product_allowed_values_map_color
                        .get_mut(&product_id)
                        .unwrap();
                    // Don't insert duplicate values
                    let color = line.color.unwrap();
                    if !allowed_values_map.contains(&color) {
                        allowed_values_map.push(color);
                    }
                }
            } else {
                if line.color.is_some() {
                    let mut color: Vec<String> = Vec::new();
                    color.push(line.color.unwrap());
                    product_allowed_values_map_color.insert(product_id, color);
                }
            }
            // Test to see if the record exists
            if product_allowed_values_map_size.contains_key(&product_id) {
                if line.size.is_some() {
                    debug!(
                        "product_id in map for size: {:?} for: {}",
                        &line.size, &product_id
                    );
                    let allowed_values_map = product_allowed_values_map_size
                        .get_mut(&product_id)
                        .unwrap();
                    // Don't insert duplicate values
                    let size = line.size.unwrap();
                    if !allowed_values_map.contains(&size) {
                        debug!("not a duplicate - inserting");
                        allowed_values_map.push(size);
                    }
                }
            } else {
                if line.size.is_some() {
                    debug!(
                        "first time found size: {:?} for: {}",
                        &line.size, &product_id
                    );
                    let mut size: Vec<String> = Vec::new();
                    size.push(line.size.unwrap());
                    product_allowed_values_map_size.insert(product_id, size);
                }
            }
        }
        debug!("colors: {:?}", product_allowed_values_map_color);
        debug!("sizes: {:?}", product_allowed_values_map_size);
    }
    info!(
        "product_allowed_values_map_color.len(): {}",
        product_allowed_values_map_color.len()
    );
    info!(
        "product_allowed_values_map_size.len(): {}",
        product_allowed_values_map_size.len()
    );
    // Build the Color record to write
    #[derive(Debug, Serialize)]
    enum Value {
        StringValue(String),
        NumberValue(i32),
        NoneValue(Option<String>),
    }
    for (k, v) in product_allowed_values_map_color {
        let num_colors = v.len();
        debug!("num_colors: {}", num_colors);
        let mut record: Vec<Value> = Vec::new();
        record.push(Value::NumberValue(k));
        record.push(Value::StringValue("Color".to_string()));
        record.push(Value::NumberValue(1));
        for s in v {
            record.push(Value::StringValue(s));
        }
        debug!("color record before write: {:?}", record);
        let mut i = 0;
        while i < (60 - (num_colors)) {
            record.push(Value::NoneValue(None));
            i += 1;
        }
        debug!("color record after adding commas: {:?}", record);
        writer.serialize(record)?;
    }
    for (k, v) in product_allowed_values_map_size {
        let num_sizes = v.len();
        debug!("num_colors: {}", num_sizes);
        let mut record: Vec<Value> = Vec::new();
        record.push(Value::NumberValue(k));
        record.push(Value::StringValue("Size".to_string()));
        record.push(Value::NumberValue(1));
        for s in v {
            record.push(Value::StringValue(s));
        }
        debug!("size record before write: {:?}", record);
        let mut i = 0;
        while i < (60 - (num_sizes)) {
            record.push(Value::NoneValue(None));
            i += 1;
        }
        debug!("size record after adding commas: {:?}", record);
        writer.serialize(record)?;
    }
    // Flush the records
    writer.flush()?;
    info!("Finished generating SKU Spec Assigns file");

    Ok(())
}

pub async fn gen_sku_spec_association_file_alternate(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    sku_spec_assignment_file: String,
    product_file: String,
    sku_file: String,
) -> Result<(), Box<dyn Error>> {
    // Setup the input and output files
    let in_file = File::open(sku_spec_assignment_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut sku_spec_value_assoc: Vec<SkuSpecificationValueAssignment> = Vec::new();
    let mut ref_ids: Vec<String> = Vec::new();
    let mut e = 0;
    for line in reader.deserialize() {
        match line {
            Ok(record) => {
                let sku_spec_value: SkuSpecificationValueAssignment = record;
                let ref_id = sku_spec_value.sku_ref_id.clone();
                sku_spec_value_assoc.push(sku_spec_value);
                ref_ids.push(ref_id);
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
        sku_spec_value_assoc.len(),
        e
    );
    debug!("ref_ids.len(): {}", ref_ids.len());

    info!("Staring generation of SKU Spec Association file");
    // Get a lookup HashMap for the product_ref_id for a sku_ref_id
    let product_ref_id_by_sku_ref_id_lookup = utils::create_sku_product_ref_id_lookup(sku_file);
    debug!(
        "product_ref_id_by_sku_ref_id_lookup: {:?}",
        product_ref_id_by_sku_ref_id_lookup.len()
    );
    // Get a lookup HashMap for the parent category of a product
    let product_parent_category_lookup =
        utils::create_product_parent_category_lookup(&product_file);
    debug!(
        "product_parent_category_lookkup: {:?}",
        product_parent_category_lookup.len()
    );

    // Build a category name lookup
    // let category_name_lookup =
    //     utils::create_category_name_lookup(client, &account_name, &environment).await;
    // debug!("category_name_lookup: {}", category_name_lookup.len());

    // Build category id lookup
    // let category_id_lookup =
    //     utils::create_category_id_lookup(client, &account_name, &environment).await;
    // debug!("category_id_lookup: {}", category_id_lookup.len());
    let category_id_lookup = utils::create_category_id_lookup_alternate(&product_file).await;
    debug!("category_id_lookup: {}", category_id_lookup.len());
    // Build a field id lookup fn get the fields for a category
    let field_id_lookup =
        utils::create_field_id_lookup(&category_id_lookup, client, &account_name, &environment)
            .await;
    debug!("field_id_lookup: {:?}", field_id_lookup.len());
    // Build a field value id lookup table
    let field_value_id_lookup =
        utils::create_field_value_id_lookup(&field_id_lookup, client, &account_name, &environment)
            .await;
    debug!("field_value_id_lookup: {:?}", field_value_id_lookup.len());

    // Build a Sku_id lookup fn
    let sku_id_lookup =
        utils::get_sku_ids_by_ref_ids(ref_ids, client, &account_name, &environment).await;
    debug!("sku_id_lookup: {}", sku_id_lookup.len());

    //    let mut sku_id_lookup: HashMap<String, i32> = HashMap::new();

    let mut x = 0;
    for line in sku_spec_value_assoc {
        let record: SkuSpecificationValueAssignment = line;

        // let sku_id: i32;
        // if !sku_id_lookup.contains_key(&record.sku_ref_id) {
        //     sku_id = utils::get_sku_id_by_ref_id(&record.sku_ref_id, &client, &account_name, &environment).await;
        //     sku_id_lookup.insert(record.sku_ref_id.clone(), sku_id.clone());
        // } else {
        //     debug!("sku_id_lookup hit. sku_ref_id: {} found.", record.sku_ref_id);
        //     sku_id = *sku_id_lookup.get(&record.sku_ref_id).unwrap();
        // }

        // Get the product_ref_id
        let product_ref_id = product_ref_id_by_sku_ref_id_lookup
            .get(&record.sku_ref_id)
            .unwrap();
        // Get the category identifier for the partnumber
        // Note: category id is same as category_identfiier
        let parent_category_identifier =
            product_parent_category_lookup.get(product_ref_id).unwrap();
        // Get category name
        // let parent_cat_name = category_name_lookup
        //     .get(parent_category_identifier)
        //     .unwrap();
        // Get the VTEX Category Id
        // let vtex_cat_id = category_id_lookup.get(parent_cat_name).unwrap();
        let vtex_cat_id = parent_category_identifier.parse::<i32>().unwrap();
        // Build the key to use with field_id_lookup
        let key = vtex_cat_id.to_string().to_owned() + "|" + record.name.as_str();
        // Get the field_id
        let field_id = field_id_lookup
            .get(&key)
            .expect("failed to find field_id in field_id_lookup");
        // Build the key to use with the field_value_id_lookup
        let field_value_key =
            field_id.to_string().as_str().to_owned() + "|" + record.value.as_str().trim();
        let field_value_id = field_value_id_lookup
            .get(&field_value_key)
            .expect("failed to find field_value_id in field_value_id_lookup");
        debug!("record.sku_ref_id {}", &record.sku_ref_id);
        // if sku_id_lookup.contains_key(&record.sku_ref_id) {
        let sku_spec_assign = SkuSpecificationAssociation {
            id: Some(0), // Hardcode to 0, API does not work with None (null)
            sku_id: *sku_id_lookup.get(&record.sku_ref_id).unwrap(),
            field_id: *field_id,
            field_value_id: Some(*field_value_id),
            text: None,
        };
        writer.serialize(sku_spec_assign)?;
        x += 1;
        // }
    }
    // Flush the records
    writer.flush()?;
    info!("records written: {}", x);
    info!("Finished generating SKU Spec Association file");

    Ok(())
}

pub async fn gen_sku_spec_association_file(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    sku_spec_assignment_file: String,
    product_file: String,
    sku_file: String,
) -> Result<(), Box<dyn Error>> {
    // Setup the input and output files
    let in_file = File::open(sku_spec_assignment_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut sku_spec_value_assoc: Vec<SkuSpecificationValueAssignment> = Vec::new();
    let mut ref_ids: Vec<String> = Vec::new();
    let mut e = 0;
    for line in reader.deserialize() {
        match line {
            Ok(record) => {
                let sku_spec_value: SkuSpecificationValueAssignment = record;
                let ref_id = sku_spec_value.sku_ref_id.clone();
                sku_spec_value_assoc.push(sku_spec_value);
                ref_ids.push(ref_id);
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
        sku_spec_value_assoc.len(),
        e
    );
    debug!("ref_ids.len(): {}", ref_ids.len());

    info!("Staring generation of SKU Spec Association file");
    // Build a category name lookup
    let category_name_lookup =
        utils::create_category_name_lookup(client, &account_name, &environment).await;
    debug!("category_name_lookup: {}", category_name_lookup.len());

    // Build a Sku_id lookup fn
    let sku_id_lookup =
        utils::get_sku_ids_by_ref_ids(ref_ids, client, &account_name, &environment).await;
    debug!("sku_id_lookup: {}", sku_id_lookup.len());
    // Get a lookup HashMap for the product_ref_id for a sku_ref_id
    let product_ref_id_by_sku_ref_id_lookup = utils::create_sku_product_ref_id_lookup(sku_file);
    debug!(
        "product_ref_id_by_sku_ref_id_lookup: {:?}",
        product_ref_id_by_sku_ref_id_lookup.len()
    );
    // Get a lookup HashMap for the parent category of a product
    let product_parent_category_lookup =
        utils::create_product_parent_category_lookup(&product_file);
    debug!(
        "product_parent_category_lookkup: {:?}",
        product_parent_category_lookup.len()
    );

    // Build category id lookup
    let category_id_lookup =
        utils::create_category_id_lookup(client, &account_name, &environment).await;
    debug!("category_id_lookup: {}", category_id_lookup.len());
    // Build a field id lookup fn get the fields for a category
    let field_id_lookup =
        utils::create_field_id_lookup(&category_id_lookup, client, &account_name, &environment)
            .await;
    debug!("field_id_lookup: {:?}", field_id_lookup.len());
    // Build a field value id lookup table
    let field_value_id_lookup =
        utils::create_field_value_id_lookup(&field_id_lookup, client, &account_name, &environment)
            .await;
    debug!("field_value_id_lookup: {:?}", field_value_id_lookup.len());

    //    let mut sku_id_lookup: HashMap<String, i32> = HashMap::new();

    let mut x = 0;
    for line in sku_spec_value_assoc {
        let record: SkuSpecificationValueAssignment = line;

        // let sku_id: i32;
        // if !sku_id_lookup.contains_key(&record.sku_ref_id) {
        //     sku_id = utils::get_sku_id_by_ref_id(&record.sku_ref_id, &client, &account_name, &environment).await;
        //     sku_id_lookup.insert(record.sku_ref_id.clone(), sku_id.clone());
        // } else {
        //     debug!("sku_id_lookup hit. sku_ref_id: {} found.", record.sku_ref_id);
        //     sku_id = *sku_id_lookup.get(&record.sku_ref_id).unwrap();
        // }

        // Get the product_ref_id
        let product_ref_id = product_ref_id_by_sku_ref_id_lookup
            .get(&record.sku_ref_id)
            .unwrap();
        // Get the category identifier for the partnumber
        let parent_category_identifier =
            product_parent_category_lookup.get(product_ref_id).unwrap();
        // Get category name
        let parent_cat_name = category_name_lookup
            .get(parent_category_identifier)
            .unwrap();
        // Get the VTEX Category Id
        let vtex_cat_id = category_id_lookup.get(parent_cat_name).unwrap();
        // Build the key to use with field_id_lookup
        let key = vtex_cat_id.to_string().to_owned() + "|" + record.name.as_str();
        // Get the field_id
        let field_id = field_id_lookup
            .get(&key)
            .expect("failed to find field_id in field_id_lookup");
        // Build the key to use with the field_value_id_lookup
        let field_value_key =
            field_id.to_string().as_str().to_owned() + "|" + record.value.as_str().trim();
        let field_value_id = field_value_id_lookup
            .get(&field_value_key)
            .expect("failed to find field_value_id in field_value_id_lookup");
        debug!("record.sku_ref_id {}", &record.sku_ref_id);
        // if sku_id_lookup.contains_key(&record.sku_ref_id) {
        let sku_spec_assign = SkuSpecificationAssociation {
            id: Some(0), // Hardcode to 0, API does not work with None (null)
            sku_id: *sku_id_lookup.get(&record.sku_ref_id).unwrap(),
            field_id: *field_id,
            field_value_id: Some(*field_value_id),
            text: None,
        };
        writer.serialize(sku_spec_assign)?;
        x += 1;
        // }
    }
    // Flush the records
    writer.flush()?;
    info!("records written: {}", x);
    info!("Finished generating SKU Spec Association file");

    Ok(())
}

pub async fn load_sku_spec_associations(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    rate_limit: NonZeroU32,
) -> Result<(), Box<dyn Error>> {
    info!("Starting load of SKU Spec Associations");
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

                let response = client.post(url).json(&record).send().await?;

                let status = response.status();
                info!(
                    "product: {:?}  text: {:?}:  response: {:?}",
                    record.sku_id, record.text, status
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
                Ok(b) => info!("output: {:?}", b),
                Err(e) => error!("error: {:?}", e),
            }
        })
        .await;

    info!("finished load of SKU Spec Associations");

    Ok(())
}
