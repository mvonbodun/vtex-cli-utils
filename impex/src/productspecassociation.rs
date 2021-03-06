use futures::{executor::block_on, stream, StreamExt};
use governor::{Jitter, Quota, RateLimiter};
use log::*;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;
use std::env;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, fs::File};
use vtex::csvrecords::ProductSpecificationAssignmentAlternate;
use vtex::model::{ProductSpecificationAssignment, ProductSpecificationAssocation};
use vtex::utils;

pub async fn gen_product_spec_association_file_root_category(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    prod_specs_assignment_file: String,
) -> Result<(), Box<dyn Error>> {
    info!("Begin gen_product_spec_association_file_root_category()");
    let mut category_lookup: HashMap<String, i32> = HashMap::new();
    category_lookup.insert("Root Category".to_string(), 0);
    debug!("category_lookup: {:?}", category_lookup.len());

    // Setup the input and output files
    debug!("current_directory: {:?}", env::current_dir());
    let in_file = File::open(prod_specs_assignment_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    // Read the input file and verify it can be deserialized, reject records that can't
    let mut prod_specs: Vec<ProductSpecificationAssignmentAlternate> = Vec::new();
    let mut e = 0;
    for line in reader.deserialize() {
        match line {
            Ok(record) => {
                let prod_spec: ProductSpecificationAssignmentAlternate = record;
                prod_specs.push(prod_spec);
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
        prod_specs.len(),
        e
    );

    // Need HashMap to get Field Id
    let field_id_lookup =
        utils::create_field_id_lookup(&category_lookup, client, &account_name, &environment).await;
    debug!("field_id_lookup: {:?}", field_id_lookup);

    for line in prod_specs {
        let record = line;

        debug!("record: {:?}", record);

        if record.short_desc.is_some() {
            debug!(
                "Found ShortDesc for product_ref_id: {}",
                record.product_ref_id
            );
            let short_desc: ProductSpecificationAssocation = ProductSpecificationAssocation {
                // Hardcode 0. If None (null), then the Post API fails with a parseInt error
                id: Some(0),
                product_id: record.product_ref_id,
                field_id: *field_id_lookup.get("0|ShortDesc:").unwrap(),
                field_value_id: None,
                text: Some(record.short_desc.unwrap()),
            };
            writer.serialize(short_desc)?;
        }
        if record.ship_message.is_some() {
            debug!(
                "Found ship_message for product_ref_id: {}",
                record.product_ref_id
            );
            let ship_message: ProductSpecificationAssocation = ProductSpecificationAssocation {
                // Hardcode 0. If None (null), then the Post API fails with a parseInt error
                id: Some(0),
                product_id: record.product_ref_id,
                field_id: *field_id_lookup.get("0|ship_message:").unwrap(),
                field_value_id: None,
                text: Some(record.ship_message.unwrap()),
            };
            writer.serialize(ship_message)?;
        }
        if record.availability_remarks.is_some() {
            debug!(
                "Found Availability Remarks for product_ref_id: {}",
                record.product_ref_id
            );
            let availability_remarks: ProductSpecificationAssocation =
                ProductSpecificationAssocation {
                    // Hardcode 0. If None (null), then the Post API fails with a parseInt error
                    id: Some(0),
                    product_id: record.product_ref_id,
                    field_id: *field_id_lookup.get("0|Availability Remarks:").unwrap(),
                    field_value_id: None,
                    text: Some(record.availability_remarks.unwrap()),
                };
            writer.serialize(availability_remarks)?;
        }
        if record.weight.is_some() {
            debug!(
                "Found Weight Remarks for product_ref_id: {}",
                record.product_ref_id
            );
            let weight: ProductSpecificationAssocation = ProductSpecificationAssocation {
                // Hardcode 0. If None (null), then the Post API fails with a parseInt error
                id: Some(0),
                product_id: record.product_ref_id,
                field_id: *field_id_lookup.get("0|Weight:").unwrap(),
                field_value_id: None,
                text: Some(record.weight.unwrap()),
            };
            writer.serialize(weight)?;
        }
        if record.package_dimensions.is_some() {
            debug!(
                "Found Package Dimensions for product_ref_id: {}",
                record.product_ref_id
            );
            let package_dimensions: ProductSpecificationAssocation =
                ProductSpecificationAssocation {
                    // Hardcode 0. If None (null), then the Post API fails with a parseInt error
                    id: Some(0),
                    product_id: record.product_ref_id,
                    field_id: *field_id_lookup.get("0|Package Dimensions:").unwrap(),
                    field_value_id: None,
                    text: Some(record.package_dimensions.unwrap()),
                };
            writer.serialize(package_dimensions)?;
        }
        if record.shipping_remarks.is_some() {
            debug!(
                "Found Shipping Remarks for product_ref_id: {}",
                record.product_ref_id
            );
            let shipping_remarks: ProductSpecificationAssocation = ProductSpecificationAssocation {
                // Hardcode 0. If None (null), then the Post API fails with a parseInt error
                id: Some(0),
                product_id: record.product_ref_id,
                field_id: *field_id_lookup.get("0|Shipping Remarks:").unwrap(),
                field_value_id: None,
                text: Some(record.shipping_remarks.unwrap()),
            };
            writer.serialize(shipping_remarks)?;
        }
        if record.prop_65.is_some() {
            debug!("Found Prop65 for product_ref_id: {}", record.product_ref_id);
            let prop_65: ProductSpecificationAssocation = ProductSpecificationAssocation {
                // Hardcode 0. If None (null), then the Post API fails with a parseInt error
                id: Some(0),
                product_id: record.product_ref_id,
                field_id: *field_id_lookup.get("0|Prop65:").unwrap(),
                field_value_id: None,
                text: Some(record.prop_65.unwrap()),
            };
            writer.serialize(prop_65)?;
        }
        if record.attachment.is_some() {
            debug!(
                "Found Attachmentfor product_ref_id: {}",
                record.product_ref_id
            );
            let attachment: ProductSpecificationAssocation = ProductSpecificationAssocation {
                // Hardcode 0. If None (null), then the Post API fails with a parseInt error
                id: Some(0),
                product_id: record.product_ref_id,
                field_id: *field_id_lookup.get("0|Attachment").unwrap(),
                field_value_id: None,
                text: Some(record.attachment.unwrap()),
            };
            writer.serialize(attachment)?;
        }
    }

    // Flush the records
    writer.flush()?;
    info!("Finished generating Product Spec Association file");

    Ok(())
}

pub async fn gen_product_spec_association_file(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    prod_specs_assignment_file: String,
    product_file: String,
) -> Result<(), Box<dyn Error>> {
    info!("Starting generate product spec assoocation file");
    // Read in the Specificaiton Groups and store in a HashMap for lookup
    let groups = utils::get_vtex_field_groups(client, &account_name, &environment).await;
    let group_lookup = utils::parse_spec_groups(groups);
    debug!("group_lookup: {:?}", group_lookup.len());

    // Read in the category tree and store in a HashMap for lookup
    let categories = utils::get_vtex_category_tree(client, &account_name, &environment).await;
    let category_lookup = utils::parse_category_tree(categories);
    debug!("category_lookup: {:?}", category_lookup.len());

    // Need HashMap to get Field Id
    let field_id_lookup =
        utils::create_field_id_lookup(&category_lookup, client, &account_name, &environment).await;
    debug!("field_id_lookup: {:?}", field_id_lookup.len());

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

    let mut product_lookup: HashMap<String, i32> = HashMap::new();

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
        debug!(
            "Product Ref Id: {}  parent_cat_name: {}",
            &record.product_ref_id,
            &parent_cat_name.to_string()
        );
        let vtex_cat_id = category_lookup.get(&parent_cat_name.to_string()).unwrap();
        debug!("vtex_cat_id: {}", vtex_cat_id);
        // Name starts in the Column 2 - index starts at 0 so position 1
        let name = record.name;
        let key = vtex_cat_id.to_string().to_owned() + "|" + name.as_str();
        let field_id = field_id_lookup
            .get(&key)
            .expect("failed to find field_id for category in field_id_lookup");

        let mut product_id: i32 = 0;
        if !product_lookup.contains_key(&record.product_ref_id) {
            let get_product_id = utils::get_product_by_ref_id(
                &record.product_ref_id,
                client,
                &account_name,
                &environment,
            )
            .await;
            match get_product_id {
                Ok(product_id_ok) => {
                    product_lookup.insert(record.product_ref_id.clone(), product_id_ok);
                    product_id = product_id_ok;
                }
                Err(err) => {
                    error!("Error: Product record will be skipped: {}", err);
                }
            }
        } else {
            debug!(
                "product_lookup hit. product_ref_id: {} found.",
                record.product_ref_id
            );
            product_id = *product_lookup.get(&record.product_ref_id).unwrap();
        }

        if product_id > 0 {
            let prod_spec: ProductSpecificationAssocation = ProductSpecificationAssocation {
                // Hardcode 0. If None (null), then the Post API fails with a parseInt error
                id: Some(0),
                product_id,
                field_id: *field_id,
                field_value_id: None,
                text: Some(record.value),
            };
            writer.serialize(prod_spec)?;
        }
    }
    // Flush the records
    writer.flush()?;
    info!("Finished generating Product Spec Association file");

    Ok(())
}

pub async fn load_product_spec_associations(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    rate_limit: NonZeroU32,
) -> Result<(), Box<dyn Error>> {
    info!("Starting product spec association load");
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/product/{productId}/specification"
    .replace("{accountName}", &account_name)
    .replace("{environment}", &environment);

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut prodspecassoc_rec: Vec<ProductSpecificationAssocation> = Vec::new();

    for line in rdr.deserialize() {
        let record: ProductSpecificationAssocation = line?;
        debug!("ProductSpecification Record: {:?}", record);
        prodspecassoc_rec.push(record);
    }

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));

    let bodies = stream::iter(prodspecassoc_rec)
        .map(|record| {
            let client = &client;
            let url = &url;
            let lim = Arc::clone(&lim);
            async move {
                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));

                let url = url.replace("{productId}", record.product_id.to_string().as_str());

                let response = client.post(url).json(&record).send().await?;

                let status = response.status();
                info!(
                    "product: {:?}  text: {:?}:  response: {:?}",
                    record.product_id, record.text, status
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

    info!("finished product spec association load");

    Ok(())
}
