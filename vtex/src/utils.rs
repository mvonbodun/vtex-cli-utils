use crate::model::{
    BrandList, Category, CategoryTree, FieldValueList, Product, Sku, SkuAndContext,
    SpecificationGroup, SpecificationList,
};
// use futures::task::Spawn;
use futures::{stream, StreamExt};
use log::*;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::time::Instant;
// use crate::csvrecords::{CatRecord, ProdHeaderRecord, ProductLookup, SkuLookup};

const CONCURRENT_REQUESTS: usize = 12;

// Get the in the Field Groups to store the Id and Name, store in a HashMap
pub async fn get_vtex_field_groups(
    client: &Client,
    account_name: &str,
    environment: &str,
) -> Vec<SpecificationGroup> {
    let url = "https://{accountName}.{environment}.com.br/api/catalog_system/pvt/specification/groupbycategory/0"
            .replace("{accountName}", account_name)
            .replace("{environment}", environment);
    let groups: Vec<SpecificationGroup> =
        client.get(url).send().await.unwrap().json().await.unwrap();
    groups
}

// Get the VTEX Category Tree - to store the Id and Name in a HashMap
pub async fn get_vtex_category_tree(
    client: &Client,
    account_name: &str,
    environment: &str,
) -> Vec<CategoryTree> {
    // TODO: Fix that this is hardcoded to 5 levels
    let url = "https://{accountName}.{environment}.com.br/api/catalog_system/pub/category/tree/5"
        .replace("{accountName}", account_name)
        .replace("{environment}", environment);
    let get_category_tree = client.get(&url).send().await;
    let response = match get_category_tree {
        Ok(result) => {
            if result.status() == StatusCode::OK {
                let category_tree: Vec<CategoryTree> = result.json().await.unwrap();
                Ok(category_tree)
            } else {
                let status = result.status().clone();
                let other_errors = result.text().await;
                match other_errors {
                    Ok(s) => {
                        let error = format!("response: {}  message: {}", status, s);
                        Err(error)
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
        }
        Err(err) => {
            // Because we sometimes get incomplete errors, connection terminated, try again
            error!(
                "Connection error on get_category_tree() {:?}",
                err.to_string()
            );
            Err(err.to_string())
        }
    };
    if response.is_err() {
        // Try again since likely a timeout
        let get_category_tree = client.get(&url).send().await;
        let response = match get_category_tree {
            Ok(result) => {
                if result.status() == StatusCode::OK {
                    let category_tree: Vec<CategoryTree> = result.json().await.unwrap();
                    category_tree
                } else {
                    let status = result.status().clone();
                    let other_errors = result.text().await;
                    match other_errors {
                        Ok(s) => {
                            let error = format!("response: {}  message: {}", status, s);
                            panic!("Failed second time get_category_by_id(): {:?}", error);
                        }
                        Err(e) => {
                            panic!("Failed second time get_category_by_id(): {:?}", e);
                        }
                    }
                }
            }
            Err(err) => {
                // Because we sometimes get incomplete errors, connection terminated, try again
                panic!(
                    "Connection error on get_category_tree() {:?}",
                    err.to_string()
                );
            }
        };
        response
    } else {
        response.unwrap()
    }
}

// Get the VTEX Category by Id
pub async fn get_category_by_id(
    client: &Client,
    account_name: &str,
    environment: &str,
    id: &i32,
) -> Category {
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/category/"
        .replace("{accountName}", account_name)
        .replace("{environment}", environment)
        + id.to_string().as_str();
    // let category: Category =
    //     client.get(url).send().await.unwrap().json().await.unwrap();
    let get_category_result = client.get(&url).send().await;
    let response = match get_category_result {
        Ok(result) => {
            if result.status() == StatusCode::OK {
                let category: Category = result.json().await.unwrap();
                Ok(category)
            } else {
                let status = result.status().clone();
                let other_errors = result.text().await;
                match other_errors {
                    Ok(s) => {
                        let error = format!("response: {}  message: {}", status, s);
                        Err(error)
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
        }
        Err(err) => {
            // Because we sometimes get incomplete errors, connection terminated, try again
            error!(
                "Connection error on get_category_by_id() {:?}",
                err.to_string()
            );
            Err(err.to_string())
        }
    };
    if response.is_err() {
        // Try again due to connection error
        let get_category_result = client.get(&url).send().await;
        let response = match get_category_result {
            Ok(result) => {
                if result.status() == StatusCode::OK {
                    let category: Category = result.json().await.unwrap();
                    category
                } else {
                    let status = result.status().clone();
                    let other_errors = result.text().await;
                    match other_errors {
                        Ok(s) => {
                            let error = format!("response: {}  message: {}", status, s);
                            panic!("Failed second time get_category_by_id(): {:?}", error);
                        }
                        Err(e) => {
                            panic!("Failed second time get_category_by_id(): {:?}", e);
                        }
                    }
                }
            }
            Err(err) => {
                // Because we sometimes get incomplete errors, connection terminated, try again
                panic!(
                    "Connection error on get_category_by_id() {:?}",
                    err.to_string()
                );
            }
        };
        response
    } else {
        // Return the category
        response.unwrap()
    }
}

// Get the specs for a given category
pub async fn get_spec_fields_for_category(
    client: &Client,
    account_name: &str,
    environment: &str,
    category_id: &str,
) -> Vec<SpecificationList> {
    let url = "https://{accountName}.{environment}.com.br/api/catalog_system/pub/specification/field/listByCategoryId/"
            .replace("{accountName}", account_name)
            .replace("{environment}", environment)
            + category_id;
    let specs: Vec<SpecificationList> = client.get(url).send().await.unwrap().json().await.unwrap();
    specs
}

// Get the field values for a given field
pub async fn get_field_values_for_field_id(
    client: &Client,
    account_name: &str,
    environment: &str,
    field_id: &str,
) -> Vec<FieldValueList> {
    let url = "https://{accountName}.{environment}.com.br/api/catalog_system/pub/specification/fieldvalue/"
            .replace("{accountName}", account_name)
            .replace("{environment}", environment)
            + field_id;
    let fieldvalues: Vec<FieldValueList> =
        client.get(url).send().await.unwrap().json().await.unwrap();
    fieldvalues
}

// Get the brands
pub async fn get_brands(client: &Client, account_name: &str, environment: &str) -> Vec<BrandList> {
    let url = "https://{accountName}.{environment}.com.br/api/catalog_system/pvt/brand/list"
        .replace("{accountName}", account_name)
        .replace("{environment}", environment);
    let response = client.get(url).send().await.unwrap();
    match response.status() {
        StatusCode::OK => {
            let result: Vec<BrandList> = response.json().await.unwrap();
            debug!("Vec<Brand> length: {}", result.len());
            result
        }
        _ => {
            debug!(
                "response.status: {}, error: {:#?}",
                response.status(),
                response.text().await.unwrap()
            );
            panic!("failed to get brands");
        }
    }
}

// Parse the Brands into a HashMap for Key Lookup
pub fn parse_brands(brands: Vec<BrandList>) -> HashMap<String, i32> {
    let mut brand_ids: HashMap<String, i32> = HashMap::new();
    for brand in brands {
        brand_ids.insert(brand.name.clone(), brand.id);
    }
    brand_ids
}

// Create brand lookup
pub async fn create_brand_lookup(
    client: &Client,
    account_name: &str,
    environment: &str,
) -> HashMap<String, i32> {
    parse_brands(get_brands(client, account_name, environment).await)
}

// Parse the Specification Groups into a HashMap for Key Lookup
pub fn parse_spec_groups(groups: Vec<SpecificationGroup>) -> HashMap<String, i32> {
    let mut group_ids: HashMap<String, i32> = HashMap::new();
    for group in groups {
        group_ids.insert(group.name.clone(), group.id.unwrap());
    }
    group_ids
}

// Read in the Category Id
// Parse the Category Tree into a HashMap for Key Lookup
pub fn parse_category_tree(cat_tree: Vec<CategoryTree>) -> HashMap<String, i32> {
    let mut category_ids: HashMap<String, i32> = HashMap::new();
    for category in cat_tree {
        category_ids.insert(category.name.clone(), category.id);
        if category.has_children {
            for category2 in category.children.expect("missing category") {
                category_ids.insert(category2.name.clone(), category2.id);
                if category2.has_children {
                    for category3 in category2.children.expect("missing category") {
                        category_ids.insert(category3.name.clone(), category3.id);
                        if category3.has_children {
                            for category4 in category3.children.expect("missing category") {
                                category_ids.insert(category4.name.clone(), category4.id);
                                if category4.has_children {
                                    for category5 in category4.children.expect("missing category") {
                                        category_ids.insert(category5.name.clone(), category5.id);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    category_ids
}

// Create category id lookup HashMap alternate version
// Use when the category id is the same as the category unique id
pub async fn create_category_id_lookup_alternate(product_file: &str) -> HashMap<String, i32> {
    let file = File::open(product_file).unwrap();
    let mut reader = csv::Reader::from_reader(file);
    let mut category_id_lookup: HashMap<String, i32> = HashMap::new();

    for line in reader.deserialize() {
        let record: Product = line.unwrap();
        category_id_lookup.insert(
            record.category_unique_identifier.clone().unwrap(),
            record.category_id.clone().unwrap(),
        );
    }
    debug!(
        "HashMap Category Identifiers Cat Id lookup alternate: {:?}",
        category_id_lookup
    );
    category_id_lookup
}

// Create category id lookup HashMap
pub async fn create_category_id_lookup(
    client: &Client,
    account_name: &str,
    environment: &str,
) -> HashMap<String, i32> {
    parse_category_tree(get_vtex_category_tree(client, account_name, environment).await)
}

// Create a lookup HashMap that allows lookup of Category Name from Category GroupIdentifier
pub async fn create_category_name_lookup(
    client: &Client,
    account_name: &str,
    environment: &str,
) -> HashMap<String, String> {
    let cat_tree = get_vtex_category_tree(client, account_name, environment).await;
    let mut cat_name_lookup: HashMap<String, String> = HashMap::new();

    for category in cat_tree {
        cat_name_lookup.insert(
            get_category_by_id(client, account_name, environment, &category.id)
                .await
                .ad_words_remarketing_code
                .unwrap(),
            category.name.clone(),
        );
        if category.has_children {
            for category2 in category.children.expect("missing category") {
                cat_name_lookup.insert(
                    get_category_by_id(client, account_name, environment, &category2.id)
                        .await
                        .ad_words_remarketing_code
                        .unwrap(),
                    category2.name.clone(),
                );
                if category2.has_children {
                    for category3 in category2.children.expect("missing category") {
                        cat_name_lookup.insert(
                            get_category_by_id(client, account_name, environment, &category3.id)
                                .await
                                .ad_words_remarketing_code
                                .unwrap(),
                            category3.name.clone(),
                        );
                        if category3.has_children {
                            for category4 in category3.children.expect("missing category") {
                                cat_name_lookup.insert(
                                    get_category_by_id(
                                        client,
                                        account_name,
                                        environment,
                                        &category4.id,
                                    )
                                    .await
                                    .ad_words_remarketing_code
                                    .unwrap(),
                                    category4.name.clone(),
                                );
                                if category4.has_children {
                                    for category5 in category4.children.expect("missing category") {
                                        cat_name_lookup.insert(
                                            get_category_by_id(
                                                client,
                                                account_name,
                                                environment,
                                                &category5.id,
                                            )
                                            .await
                                            .ad_words_remarketing_code
                                            .unwrap(),
                                            category5.name.clone(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cat_name_lookup
}

// Create a lookup HashMap that allows lookup of the parent category_unique_identifier by the product ref_id
pub fn create_product_parent_category_lookup(product_file: &str) -> HashMap<String, String> {
    let file = File::open(product_file).unwrap();
    let mut reader = csv::Reader::from_reader(file);
    let mut product_parent_category: HashMap<String, String> = HashMap::new();

    for line in reader.deserialize() {
        let record: Product = line.unwrap();
        product_parent_category.insert(
            record.ref_id.unwrap().clone(),
            record.category_unique_identifier.clone().unwrap(),
        );
    }
    debug!(
        "HashMap Category Identifiers: {:?}",
        product_parent_category
    );
    product_parent_category
}

// Create a lookup HashMap that allows lookup of the product_ref_id by the sku_ref_id
pub fn create_sku_product_ref_id_lookup(sku_file: String) -> HashMap<String, String> {
    let file = File::open(&sku_file).unwrap();
    let mut reader = csv::Reader::from_reader(file);
    let mut sku_product_ref_id_lookup: HashMap<String, String> = HashMap::new();

    for line in reader.deserialize() {
        let record: Sku = line.unwrap();
        sku_product_ref_id_lookup.insert(record.ref_id.clone(), record.product_ref_id.clone());
    }
    debug!(
        "HashMap Sku Product ref_id lookup: {:?}",
        sku_product_ref_id_lookup
    );
    sku_product_ref_id_lookup
}

//     // Create a lookup HashMap that allows lookup of the parent category GroupoIdentifier by the PartNumber
//     pub fn create_sku_parent_category_lookup() -> HashMap<String, String> {
//         let file = File::open("transform/data/in/ProductHeaderItem-sorted-subset.csv").unwrap();
//         let mut reader = csv::Reader::from_reader(file);
//         let mut sku_parent_category: HashMap<String, String> = HashMap::new();

//         for line in reader.deserialize() {
//             let record: ProdHeaderRecord = line.unwrap();
//             sku_parent_category.insert(
//                 record.part_number.clone(),
//                 record.parent_group_identifier.clone(),
//             );
//         }
//         // println!("HashMap Category Identifiers: {:?}", sku_parent_category);
//         sku_parent_category
//     }

// Create field_id lookup.  key =
pub async fn create_field_id_lookup(
    category_lookup: &HashMap<String, i32>,
    client: &Client,
    account_name: &str,
    environment: &str,
) -> HashMap<String, i32> {
    // Lookup by [cat_id + field name, field-id]
    let mut field_id_lookup: HashMap<String, i32> = HashMap::new();
    for category in category_lookup {
        // get the fields for the category
        let category_fields = get_spec_fields_for_category(
            client,
            account_name,
            environment,
            category.1.to_string().as_str(),
        )
        .await;
        for cat_field in category_fields {
            let key = category.1.to_string().as_str().to_owned() + "|" + cat_field.name.as_str();
            field_id_lookup.insert(key, cat_field.field_id);
        }
    }
    field_id_lookup
}

// Get Product by RefId
pub async fn get_product_by_ref_id(
    ref_id: &str,
    client: &Client,
    account_name: &str,
    environment: &str,
) -> Result<i32, String> {
    let url = "https://{accountName}.{environment}.com.br/api/catalog_system/pvt/products/productgetbyrefid/"
            .replace("{accountName}", account_name)
            .replace("{environment}", environment)
            + ref_id;

    let product_id_result = client.get(url).send().await;
    let response = match product_id_result {
        Ok(result) => {
            if result.status() == StatusCode::OK {
                // This API returns a 200 even if not found. The body contains "null"
                let result = result.json::<Product>().await;
                match result {
                    Ok(product) => Ok(product.id.unwrap()),
                    Err(e) => {
                        let error =
                            format!("product with ref_id: {} not found. error: {}", ref_id, e);
                        Err(error)
                    }
                }
            } else if result.status() == StatusCode::NOT_FOUND {
                let error = format!(
                    "response: {} product with ref_id: {} not found",
                    result.status(),
                    ref_id
                );
                Err(error)
            } else {
                let status = result.status().clone();
                let other_errors = result.text().await;
                match other_errors {
                    Ok(s) => {
                        let error = format!("response: {}  message: {}", status, s);
                        Err(error)
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
        }
        Err(err) => Err(err.to_string()),
    };
    response
}

// Get Sku by RefId
pub async fn get_sku_id_by_ref_id(
    ref_id: &str,
    client: &Client,
    account_name: &str,
    environment: &str,
) -> Result<i32, String> {
    let url = "https://{accountName}.{environment}.com.br/api/catalog_system/pvt/sku/stockkeepingunitidbyrefid/"
            .replace("{accountName}", account_name)
            .replace("{environment}", environment)
            + ref_id;

    // let sku_id_result = client.get(url).send().await.unwrap().json().await.unwrap();
    let sku_id_result = client.get(url).send().await;
    let response = match sku_id_result {
        Ok(result) => {
            if result.status() == StatusCode::OK {
                let sku_id: String = result.json().await.unwrap();
                Ok(sku_id.parse::<i32>().unwrap())
            } else if result.status() == StatusCode::NOT_FOUND {
                let error = format!(
                    "response: {} sku with ref_id: {} not found",
                    result.status(),
                    ref_id
                );
                Err(error)
            } else {
                let status = result.status().clone();
                let other_errors = result.text().await;
                match other_errors {
                    Ok(s) => {
                        let error = format!("response: {}  message: {}", status, s);
                        Err(error)
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
        }
        Err(err) => Err(err.to_string()),
    };
    response
}

// Get Sku Ids by RefIds
pub async fn get_sku_ids_by_ref_ids(
    ref_ids: Vec<String>,
    client: &Client,
    account_name: &str,
    environment: &str,
) -> HashMap<String, i32> {
    // Build the URL's to the the sku_id's for the ref_id's passed in
    let url = "https://{accountName}.{environment}.com.br/api/catalog_system/pvt/sku/stockkeepingunitidbyrefid/{refId}"
            .replace("{accountName}", account_name)
            .replace("{environment}", environment);
    let mut urls: Vec<String> = Vec::with_capacity(ref_ids.len());
    for ref_id in ref_ids {
        let url = url.replace("{refId}", ref_id.to_string().as_str());
        urls.push(url);
    }
    debug!("sku urls.len(): {}", urls.len());
    struct SkuIdentifiers {
        ref_id: String,
        sku_id: String,
    }

    let item_lookup: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let sku_id_result = client.get(url.clone()).send().await;
                // Get the ref_id from the end of the URL
                // Use split logic because some ref_id's have a / in them
                // TODO: enhance this in the future using a REGEX
                let split = url.split("/");
                let count = split.clone().count();
                let s_last = split.clone().nth(count - 1).unwrap();
                let s_next_to_last = split.clone().nth(count - 2).unwrap();
                let s_2nd_to_last = split.clone().nth(count - 3).unwrap();
                debug!(
                    "last: {:?}, next to last: {:?}, second to last: {:?} ",
                    s_last, s_next_to_last, s_2nd_to_last
                );
                let mut ref_id = String::new();
                if s_next_to_last.eq("stockkeepingunitidbyrefid") {
                    ref_id = s_last.to_string();
                } else {
                    ref_id = format!("{}{}{}", s_next_to_last, "/", s_last);
                }

                let response = match sku_id_result {
                    Ok(result) => {
                        if result.status() == StatusCode::OK {
                            let sku_id: String = result.json().await.unwrap();
                            Ok(SkuIdentifiers {
                                ref_id: ref_id,
                                sku_id,
                            })
                        } else if result.status() == StatusCode::NOT_FOUND {
                            let error = format!(
                                "response: {} sku with ref_id: {} not found",
                                result.status(),
                                ref_id
                            );
                            Err(error)
                        } else {
                            let status = result.status().clone();
                            let other_errors = result.text().await;
                            match other_errors {
                                Ok(s) => {
                                    let error = format!("response: {}  message: {}", status, s);
                                    Err(error)
                                }
                                Err(e) => Err(e.to_string()),
                            }
                        }
                    }
                    Err(err) => Err(err.to_string()),
                };
                response
            }
        })
        .buffer_unordered(2);
    bodies
        .for_each(|b| async {
            let item_lookup = item_lookup.clone();
            match b {
                Ok(b) => {
                    let sku_identifiers: SkuIdentifiers = b;
                    debug!(
                        "sku_identifiers.ref_id: {} sku_identifiers.sku_id {}",
                        sku_identifiers.ref_id, sku_identifiers.sku_id
                    );
                    let mut item_lookup = item_lookup.lock().unwrap();
                    item_lookup.insert(
                        sku_identifiers.ref_id,
                        sku_identifiers.sku_id.parse::<i32>().clone().unwrap(),
                    );
                }
                Err(e) => error!("Got an error on : {}", e),
            }
        })
        .await;

    let ir = item_lookup.lock().unwrap().clone();
    info!(
        "finished get_item_records(): item_recs.len(): {:?}",
        ir.len()
    );
    debug!("item_lookup {:?}", ir);
    ir
}

// Create field value id lookup. key = field_id + "|" + value, returns field_value_id
pub async fn create_field_value_id_lookup(
    field_id_lookup: &HashMap<String, i32>,
    client: &Client,
    account_name: &str,
    environment: &str,
) -> HashMap<String, i32> {
    let mut field_value_id_lookup: HashMap<String, i32> = HashMap::new();
    for field in field_id_lookup {
        let field_values = get_field_values_for_field_id(
            client,
            account_name,
            environment,
            field.1.to_string().as_str(),
        )
        .await;
        for field_value in field_values {
            let key = field.1.to_string().as_str().to_owned() + "|" + field_value.value.as_str();
            field_value_id_lookup.insert(key, field_value.field_value_id);
        }
    }
    field_value_id_lookup
}

pub async fn get_all_sku_ids(client: &Client, account_name: &str, environment: &str) -> Vec<i32> {
    let start = Instant::now();
    info!("Start get_all_sku_ids()");
    // Get all the skus
    let sku_ids: &mut Vec<i32> = &mut Vec::new();
    let recs = &mut 1000;
    let page = &mut 1;

    while *recs == 1000 {
        *recs = get_all_sku_ids_by_page(*page, client, account_name, environment, sku_ids).await;
        *page += 1;
    }
    let duration = start.elapsed();
    info!(
        "Finished get_all_sku_ids: {} records in {:?}",
        sku_ids.len(),
        duration
    );
    sku_ids.to_vec()
}

pub async fn get_all_sku_ids_by_page(
    page: i32,
    client: &Client,
    account_name: &str,
    environment: &str,
    sku_ids: &mut Vec<i32>,
) -> i32 {
    let url = "https://{accountName}.{environment}.com.br/api/catalog_system/pvt/sku/stockkeepingunitids?page={page}&pagesize=1000"
            .replace("{accountName}", account_name)
            .replace("{environment}", environment)    
            .replace("{page}", page.to_string().as_str());

    let response = client.get(url).send().await.unwrap();

    // println!("response.status: {}", response.status());
    match response.status() {
        StatusCode::OK => {
            let response_text = response.text().await.unwrap();
            let ids = response_text.replace("[", "").replace("]", "");
            let comma: char = ',';
            let iter = ids.split(comma);
            let mut x = 0;
            debug!("ids: {:?}", ids);
            // let mut ids_response: Vec<i32> = Vec::new();
            for v in iter {
                sku_ids.push(v.parse::<i32>().unwrap());
                x += 1;
            }
            x
        }
        _ => {
            panic!(
                "Status Code: [{:?}] Error: [{:#?}]",
                response.status(),
                response.text().await
            )
        }
    }
}

fn build_get_sku_urls(sku_ids: &[i32], account_name: &str, environment: &str) -> Vec<String> {
    let url = "https://{accountName}.{environment}.com.br/api/catalog_system/pvt/sku/stockkeepingunitbyid/{skuId}?sc=1"
            .replace("{accountName}", account_name)
            .replace("{environment}", environment);
    let mut urls: Vec<String> = Vec::with_capacity(sku_ids.len());
    for sku_id in sku_ids {
        let url = url.replace("{skuId}", sku_id.to_string().as_str());
        urls.push(url);
    }
    debug!("sku urls.len(): {}", urls.len());
    urls
}

async fn get_item_records(
    sku_ids: &[i32],
    client: &Client,
    account_name: &str,
    environment: &str,
) -> HashMap<i32, SkuAndContext> {
    info!("Starting get_item_records()");
    // Build the urls
    let urls = build_get_sku_urls(sku_ids, account_name, environment);
    debug!("after call to build_get_sku_urls");
    let item_recs: Arc<Mutex<HashMap<i32, SkuAndContext>>> = Arc::new(Mutex::new(HashMap::new()));
    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = client.get(url.clone()).send().await?;

                // let sctx: SkuAndContext = resp.json().await?;
                debug!("end of async move - url: {}", url);
                // resp.text().await
                resp.json::<SkuAndContext>().await
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);
    bodies
        .for_each(|b| async {
            let item_recs = item_recs.clone();
            match b {
                Ok(b) => {
                    // let result: Result<SkuAndContext, serde_json::Error> = serde_json::from_str(b).unwrap();
                    let sku_ctx: SkuAndContext = b;
                    let mut item_recs = item_recs.lock().unwrap();
                    item_recs.insert(sku_ctx.id, sku_ctx.clone());
                    debug!("Got: {:?}", sku_ctx)
                }
                Err(e) => error!("Got an error: {}", e),
            }
        })
        .await;

    let ir = item_recs.lock().unwrap().clone();
    info!(
        "finished get_item_records(): item_recs.len(): {:?}",
        ir.len()
    );
    ir
}

//     pub fn create_product_id_lookup() -> HashMap<String, i32> {
//         println!("env path: {:?}", env::current_dir());
//         let file =
//             File::open("data/ProductLookup.csv").expect("Did not find file data/ProductLookup.csv");
//         let mut reader = csv::Reader::from_reader(file);

//         let mut product_lookup = HashMap::new();
//         for line in reader.deserialize() {
//             let record: ProductLookup = line.unwrap();
//             product_lookup.insert(record.part_number.clone(), record.product_id.clone());
//         }
//         product_lookup
//     }

pub async fn create_sku_id_lookup(
    client: &Client,
    account_name: &str,
    environment: &str,
) -> HashMap<String, i32> {
    info!("Start creating sku_id_lookup");
    let mut sku_lookup = HashMap::new();
    let sku_ids = get_all_sku_ids(client, account_name, environment).await;
    let item_records = get_item_records(&sku_ids, client, account_name, environment).await;
    for ir in item_records {
        let sku_id = ir.0;
        let sku_context = ir.1;
        sku_lookup.insert(sku_context.alternate_ids.ref_id.clone(), sku_id);
    }
    info!(
        "Finish creating sku_id_lookup length: {:?}",
        sku_lookup.len()
    );
    sku_lookup
}

#[cfg(test)]
mod tests {
    use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

    //  #[test]
    // fn get_product_id_by_ref_id() {
    //     let url = "https://pitstopusa.vtexcommercestable.com.br/api/catalog_system/pvt/sku/stockkeepingunitidbyrefid/ALL99852";
    //     let re = Regex::new(r"/.*?byrefid(.*?)g").unwrap();
    //     let cap = re.captures(url);
    //     print!("ref_id: {:?}", &cap);
    //  }

    #[test]
    fn get_product_id_by_ref_id_split() {
        let url = "https://pitstopusa.vtexcommercestable.com.br/api/catalog_system/pvt/sku/stockkeepingunitidbyrefid/ALL99852";
        let split = url.split("/");
        println!("ref_id: {:?}", split.last());
        // for s in split {
        //     println!("{}", s);
        // }
    }

    #[test]
    fn get_product_id_by_ref_id_split_2() {
        let url = "https://pitstopusa.vtexcommercestable.com.br/api/catalog_system/pvt/sku/stockkeepingunitidbyrefid/ALP4752117-106-M/L";
        let split = url.split("/");
        let count = split.clone().count();
        let s_last = split.clone().nth(count - 1).unwrap();
        let s_next_to_last = split.clone().nth(count - 2).unwrap();
        let s_2nd_to_last = split.clone().nth(count - 3).unwrap();
        println!(
            "last: {:?}, next to last: {:?}, second to last: {:?} ",
            s_last, s_next_to_last, s_2nd_to_last
        );

        let mut s = String::new();
        if s_next_to_last.eq("stockkeepingunitidbyrefid") {
            s = s_last.to_string();
        } else {
            s = format!("{}{}{}", s_next_to_last, "/", s_last);
        }
        println!("s: {:?}", s);
        // println!("ref_id: {:?}", split.last());
        // for s in split {
        //     println!("{}", s);
        // }
    }

    #[test]
    fn percent_encode_ref_id() {
        const FRAGMENT: &AsciiSet = &CONTROLS.add(b'/');
        let ref_id = "ALP4752117-106-M/L";
        let result = utf8_percent_encode(ref_id, FRAGMENT);
        println!("result: {:?}: ", result.to_string());
    }
}

//     #[test]
//     fn find_brand_id() {
//         let brands = [
//             Brand::new(
//                 Some(1),
//                 "Nike".to_string(),
//                 Some("Nike".to_string()),
//                 Some("Nike".to_string()),
//                 Some("Nike".to_string()),
//                 true,
//                 None,
//                 None,
//                 None,
//                 None,
//             ),
//             Brand::new(
//                 Some(2),
//                 "Adidas".to_string(),
//                 Some("Adidas".to_string()),
//                 Some("Adidas".to_string()),
//                 Some("Adidas".to_string()),
//                 true,
//                 None,
//                 None,
//                 None,
//                 None,
//             ),
//             Brand::new(
//                 Some(3),
//                 "New Balance".to_string(),
//                 Some("New Balance".to_string()),
//                 Some("New Balance".to_string()),
//                 Some("New Balance".to_string()),
//                 true,
//                 None,
//                 None,
//                 None,
//                 None,
//             ),
//             Brand::new(
//                 Some(4),
//                 "Saucony".to_string(),
//                 Some("Saucony".to_string()),
//                 Some("Saucony".to_string()),
//                 Some("Saucony".to_string()),
//                 true,
//                 None,
//                 None,
//                 None,
//                 None,
//             ),
//         ];

//         let brand_id: Vec<Option<i32>> = brands
//             .iter()
//             .filter(|b| b.name.eq("New Balance"))
//             .map(|b| b.id)
//             .collect();
//         println!("brand_id: {:?}", brand_id[0].unwrap());
//     }
// }
