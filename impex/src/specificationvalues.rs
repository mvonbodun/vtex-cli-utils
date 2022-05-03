use csv::StringRecord;
use futures::{executor::block_on, stream, StreamExt};
use governor::{Jitter, Quota, RateLimiter};
use log::*;
use reqwest::{Client, StatusCode};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use vtex::model::SpecificationValue;
use vtex::utils;

pub async fn gen_specification_values_file(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    sku_spec_allowed_values_file: String,
    product_file: String,
) -> Result<(), Box<dyn Error>> {
    info!("Starting generation of specification values file");
    // Read in the category tree and store in a HashMap for lookup
    let categories = utils::get_vtex_category_tree(client, &account_name, &environment).await;
    let category_lookup = utils::parse_category_tree(categories);
    debug!("category_lookup: {:?}", category_lookup.len());

    // Need HashMap to get Field Id
    let field_id_lookup =
        utils::create_field_id_lookup(&category_lookup, client, &account_name, &environment).await;
    debug!("field_id_lookup: {:?}", field_id_lookup.len());
    // Get a lookup HashMap for the parent category of a product
    let product_parent_category_lookup = utils::create_product_parent_category_lookup(product_file);
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

    let in_file = File::open(&sku_spec_allowed_values_file).unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = file_path;
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut record = StringRecord::new();
    let mut specvalue_set: HashSet<SpecificationValue> = HashSet::new();
    while reader.read_record(&mut record)? {
        // product_ref_id is in the Column 1 - index starts at 0 so position 0
        let product_ref_id = record.get(0).unwrap().to_string();
        debug!("product_ref_id: {}", product_ref_id);
        // look up the part number
        let parent_cat_identifier = product_parent_category_lookup.get(&product_ref_id).unwrap();
        // look up the category name
        let parent_cat_name = category_identifier_name_lookup
            .get(&parent_cat_identifier.to_string())
            .unwrap();
        // Look up the VTEX Category Id
        let vtex_cat_id = category_lookup.get(&parent_cat_name.to_string()).unwrap();
        debug!("vtex_cat_id: {}", vtex_cat_id);
        // Name starts in the Column 2 - index starts at 0 so position 1
        let name = record.get(1).unwrap().to_string();
        let key = vtex_cat_id.to_string().to_owned() + "|" + name.as_str();
        let field_id = field_id_lookup
            .get(&key)
            .expect("failed to find field_id for category in field_id_lookup");

        // The AllowedValues fields start in the 4th postion of the file - range begins at 3 in for loop
        for number in 3..record.len() {
            let value = record.get(number).unwrap().trim();
            if !value.is_empty() {
                debug!("name: [{}] value: [{}]", name, value);
                let field_value = SpecificationValue {
                    field_value_id: None,
                    field_id: *field_id,
                    is_active: Some(true),
                    name: value.to_string(),
                    text: None,
                    position: None,
                };
                // Don't insert duplicate records
                if !specvalue_set.contains(&field_value) {
                    writer.serialize(field_value.clone())?;
                    specvalue_set.insert(field_value);
                }
            }
        }
    }
    // Flush the records
    writer.flush()?;
    info!("Finished specification values file generation");

    Ok(())
}

pub async fn load_specification_values(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    rate_limit: NonZeroU32,
) -> Result<(), Box<dyn Error>> {
    info!("Starting specification values load");
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/specificationvalue"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut specvalues_rec: Vec<SpecificationValue> = Vec::new();

    for line in rdr.deserialize() {
        let record: SpecificationValue = line?;
        debug!("SpecificationValue Record: {:?}", record);
        specvalues_rec.push(record);
    }

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));

    let bodies = stream::iter(specvalues_rec)
        .map(|record| {
            let client = &client;
            let url = &url;
            let lim = Arc::clone(&lim);
            async move {
                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));

                let response = client.post(url).json(&record).send().await?;

                info!("name: {:?}: response: {:?}", record.name, response.status());
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

    info!("finished loading specification values");

    Ok(())
}
