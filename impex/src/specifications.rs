use futures::{stream, StreamExt};
use log::*;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;

use reqwest::Client;
use vtex::model::{ProductSpecificationAssignment, SkuSpecAllowedValues, Specification};
use vtex::utils;

pub async fn gen_product_specifications_file(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    prod_specs_assignment_file: String,
    product_file: String,
) -> Result<(), Box<dyn Error>> {
    info!("Starting product specification file generation");
    // Read in the Specificaiton Groups and store in a HashMap for lookup
    let groups = utils::get_vtex_field_groups(client, &account_name, &environment).await;
    let group_lookup = utils::parse_spec_groups(groups);
    debug!("group_lookup: {:?}", group_lookup.len());
    // TODO: Need to figure out this hard-coded value
    let prod_spec_id = group_lookup.get("Default Specification Group").expect("Expected [Default Specification Group]. Ensure you create a specification group with this value.");

    // Read in the category tree and store in a HashMap for lookup
    let categories = utils::get_vtex_category_tree(client, &account_name, &environment).await;
    let category_lookup = utils::parse_category_tree(categories);
    debug!("category_lookup: {:?}", category_lookup.len());

    // Get a lookup HashMap for the parent category of a product
    let product_parent_category_lookup =
        utils::create_product_parent_category_lookup(&product_file);
    debug!(
        "product_parent_category_lookkup: {:?}",
        product_parent_category_lookup.len()
    );
    // Get a lookup for the cateogory name of a category by GroupIdentifier
    let category_identifier_name_lookup =
        utils::create_category_name_lookup(client, &account_name, &environment).await;
    debug!(
        "category_identifier_name_lookup: {:?}",
        category_identifier_name_lookup.len()
    );

    // Setup the input and output files
    debug!("current_directory: {:?}", env::current_dir());
    let in_file = File::open(prod_specs_assignment_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    // Create a HashSet to store unique values
    let mut unique_spec_cat: HashSet<String> = HashSet::new();

    // Process the input file
    for line in reader.deserialize() {
        let record: ProductSpecificationAssignment = line.unwrap();
        // look up the part number
        let parent_cat_identifier = product_parent_category_lookup
            .get(&record.product_ref_id)
            .unwrap();
        // look up the category name
        let parent_cat_name = category_identifier_name_lookup
            .get(&parent_cat_identifier.to_string())
            .unwrap();
        // Look up the VTEX Category Id
        // println!("PartNumber: {}  parent_cat_name: {}", &record.part_number, &parent_cat_name.to_string());
        let vtex_cat_id = category_lookup.get(&parent_cat_name.to_string()).unwrap();
        // println!("vtex_cat_id: {}", vtex_cat_id);

        // Only write a record if the Specification for the given category has not been written
        let unique_spec_cat_id: String =
            record.name.clone() + String::as_str(&vtex_cat_id.to_string());
        if !unique_spec_cat.contains(&unique_spec_cat_id) {
            debug!("unique_spec_cat_id: {}", unique_spec_cat_id);
            if !record.name.eq("Brand") {
                let spec = Specification {
                    id: None,
                    field_type_id: 1, // 1 = Text
                    category_id: Some(*vtex_cat_id),
                    field_group_id: *prod_spec_id,
                    name: record.name.clone(),
                    description: Some(record.name.clone()),
                    position: Some(record.position),
                    is_filter: Some(record.is_filter),
                    is_required: Some(record.is_required),
                    is_on_product_details: Some(record.is_on_product_details),
                    is_stock_keeping_unit: Some(record.is_stock_keeping_unit),
                    is_wizard: Some(record.is_wizard),
                    is_active: Some(record.is_active),
                    is_top_menu_link_active: Some(record.is_top_menu_link_active),
                    default_value: record.default_value,
                };

                // Write the record
                writer.serialize(spec)?;
                // Add the unique_spec_cat_id to the set
                unique_spec_cat.insert(unique_spec_cat_id);
            }
        }
    }
    // Flush the records
    writer.flush()?;
    info!("Finished generating product specification file");

    Ok(())
}

pub async fn gen_sku_specifications_file_alternate(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    sku_specs_assignment_file: String,
    product_file: String,
) -> Result<(), Box<dyn Error>> {
    info!("Starting SKU specification file generation");

    // Setup the input and output files
    debug!("current_directory: {:?}", env::current_dir());
    let in_file = File::open(sku_specs_assignment_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut sku_specs_allowed_values: Vec<SkuSpecAllowedValues> = Vec::new();
    let mut e = 0;
    for line in reader.deserialize() {
        match line {
            Ok(record) => {
                let sku_spec_allowed_value: SkuSpecAllowedValues = record;
                sku_specs_allowed_values.push(sku_spec_allowed_value);
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
        sku_specs_allowed_values.len(),
        e
    );

    // Read in the Specificaiton Groups and store in a HashMap for lookup
    let groups = utils::get_vtex_field_groups(client, &account_name, &environment).await;
    let group_lookup = utils::parse_spec_groups(groups);
    debug!("group_lookup: {:?}", group_lookup.len());
    // TODO: Need to figure out this hard-coded value
    let prod_spec_id = group_lookup.get("Default Specification Group").expect("Expected [Default Specification Group]. Ensure you create a specification group with this value.");

    // Read in the category tree and store in a HashMap for lookup
    let categories = utils::get_vtex_category_tree(client, &account_name, &environment).await;
    let category_lookup = utils::parse_category_tree(categories);
    debug!("category_lookup: {:?}", category_lookup.len());

    // Get a lookup HashMap for the parent category of a product
    let product_parent_category_lookup =
        utils::create_product_parent_category_lookup(&product_file);
    debug!(
        "product_parent_category_lookkup: {:?}",
        product_parent_category_lookup.len()
    );
    // Get a lookup for the cateogory name of a category by GroupIdentifier
    // let category_identifier_name_lookup =
    //     utils::create_category_name_lookup(client, &account_name, &environment).await;
    // debug!(
    //     "category_identifier_name_lookup: {:?}",
    //     category_identifier_name_lookup.len()
    // );

    // Create a HashSet to store unique values
    let mut unique_spec_cat: HashSet<String> = HashSet::new();

    // Process the input file
    for line in sku_specs_allowed_values {
        let record: SkuSpecAllowedValues = line;
        // look up the part number
        let parent_cat_identifier = product_parent_category_lookup
            .get(&record.product_ref_id)
            .unwrap();
        // // look up the category name
        // let parent_cat_name = category_identifier_name_lookup
        //     .get(&parent_cat_identifier.to_string())
        //     .unwrap();
        // // Look up the VTEX Category Id
        // debug!(
        //     "PartNumber: {}  parent_cat_name: {}",
        //     &record.product_ref_id,
        //     &parent_cat_name.to_string()
        // );
        let vtex_cat_id = parent_cat_identifier.parse::<i32>().unwrap();
        debug!("vtex_cat_id: {}", vtex_cat_id);

        // Only write a record if the Specification for the given category has not been written
        let unique_spec_cat_id: String =
            record.name.clone() + String::as_str(&vtex_cat_id.to_string());
        if !unique_spec_cat.contains(&unique_spec_cat_id) {
            debug!("unique_spec_cat_id: {}", unique_spec_cat_id);
            let spec = Specification {
                id: None,
                field_type_id: 6, // 6 = Radio
                category_id: Some(vtex_cat_id),
                field_group_id: *prod_spec_id,
                name: record.name.clone(),
                description: Some(record.name.clone()),
                position: Some(record.position),
                is_filter: Some(true),
                is_required: Some(false),
                is_on_product_details: Some(false),
                is_stock_keeping_unit: Some(true),
                is_wizard: Some(false),
                is_active: Some(true),
                is_top_menu_link_active: Some(false),
                default_value: None,
            };

            // Write the record
            writer.serialize(spec)?;
            // Add the unique_spec_cat_id to the set
            unique_spec_cat.insert(unique_spec_cat_id);
        }
    }
    // Flush the records
    writer.flush()?;
    info!("Finished generating SKU specification file");

    Ok(())
}

pub async fn gen_sku_specifications_file(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    sku_specs_assignment_file: String,
    product_file: String,
) -> Result<(), Box<dyn Error>> {
    info!("Starting SKU specification file generation");

    // Setup the input and output files
    debug!("current_directory: {:?}", env::current_dir());
    let in_file = File::open(sku_specs_assignment_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut sku_specs_allowed_values: Vec<SkuSpecAllowedValues> = Vec::new();
    let mut e = 0;
    for line in reader.deserialize() {
        match line {
            Ok(record) => {
                let sku_spec_allowed_value: SkuSpecAllowedValues = record;
                sku_specs_allowed_values.push(sku_spec_allowed_value);
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
        sku_specs_allowed_values.len(),
        e
    );

    // Read in the Specificaiton Groups and store in a HashMap for lookup
    let groups = utils::get_vtex_field_groups(client, &account_name, &environment).await;
    let group_lookup = utils::parse_spec_groups(groups);
    debug!("group_lookup: {:?}", group_lookup.len());
    // TODO: Need to figure out this hard-coded value
    let prod_spec_id = group_lookup.get("Default Specification Group").expect("Expected [Default Specification Group]. Ensure you create a specification group with this value.");

    // Read in the category tree and store in a HashMap for lookup
    let categories = utils::get_vtex_category_tree(client, &account_name, &environment).await;
    let category_lookup = utils::parse_category_tree(categories);
    debug!("category_lookup: {:?}", category_lookup.len());

    // Get a lookup HashMap for the parent category of a product
    let product_parent_category_lookup =
        utils::create_product_parent_category_lookup(&product_file);
    debug!(
        "product_parent_category_lookkup: {:?}",
        product_parent_category_lookup.len()
    );
    // Get a lookup for the cateogory name of a category by GroupIdentifier
    let category_identifier_name_lookup =
        utils::create_category_name_lookup(client, &account_name, &environment).await;
    debug!(
        "category_identifier_name_lookup: {:?}",
        category_identifier_name_lookup.len()
    );

    // Create a HashSet to store unique values
    let mut unique_spec_cat: HashSet<String> = HashSet::new();

    // Process the input file
    for line in sku_specs_allowed_values {
        let record: SkuSpecAllowedValues = line;
        // look up the part number
        let parent_cat_identifier = product_parent_category_lookup
            .get(&record.product_ref_id)
            .unwrap();
        // look up the category name
        let parent_cat_name = category_identifier_name_lookup
            .get(&parent_cat_identifier.to_string())
            .unwrap();
        // Look up the VTEX Category Id
        debug!(
            "PartNumber: {}  parent_cat_name: {}",
            &record.product_ref_id,
            &parent_cat_name.to_string()
        );
        let vtex_cat_id = category_lookup.get(&parent_cat_name.to_string()).unwrap();
        debug!("vtex_cat_id: {}", vtex_cat_id);

        // Only write a record if the Specification for the given category has not been written
        let unique_spec_cat_id: String =
            record.name.clone() + String::as_str(&vtex_cat_id.to_string());
        if !unique_spec_cat.contains(&unique_spec_cat_id) {
            debug!("unique_spec_cat_id: {}", unique_spec_cat_id);
            let spec = Specification {
                id: None,
                field_type_id: 6, // 6 = Radio
                category_id: Some(*vtex_cat_id),
                field_group_id: *prod_spec_id,
                name: record.name.clone(),
                description: Some(record.name.clone()),
                position: Some(record.position),
                is_filter: Some(true),
                is_required: Some(false),
                is_on_product_details: Some(false),
                is_stock_keeping_unit: Some(true),
                is_wizard: Some(false),
                is_active: Some(true),
                is_top_menu_link_active: Some(false),
                default_value: None,
            };

            // Write the record
            writer.serialize(spec)?;
            // Add the unique_spec_cat_id to the set
            unique_spec_cat.insert(unique_spec_cat_id);
        }
    }
    // Flush the records
    writer.flush()?;
    info!("Finished generating SKU specification file");

    Ok(())
}

pub async fn load_specifications(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
) -> Result<(), Box<dyn Error>> {
    info!("Starting specification load");
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/specification"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut spec_recs: Vec<Specification> = Vec::new();

    for line in rdr.deserialize() {
        let record: Specification = line?;
        spec_recs.push(record);
    }

    info!("specification records: {:?}", spec_recs.len());

    let bodies = stream::iter(spec_recs)
        .map(|record| {
            let client = &client;
            let url = &url;
            async move {
                let response = client.post(url).json(&record).send().await?;

                info!(
                    "specification : {:?}: response: {:?}",
                    record.id,
                    response.status()
                );

                response.json::<Specification>().await
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

    info!("Finished loading specifications");

    Ok(())
}
