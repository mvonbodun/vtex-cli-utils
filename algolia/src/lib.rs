use algoliarecords::{HierarchicalCategories, Variant, Price, Review};
use dotenv;
use log::*;
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::{error::Error, time::{Duration, Instant}, env, sync::Once, collections::HashMap, fs::File, io::BufWriter};
use std::io::Write;
use std::sync::{Arc, Mutex};
use futures::{join, stream, StreamExt };

use reqwest::{header, Method, Request, Url, StatusCode, Client};
use vtex::model::{SkuAndContext, Image, SkuSpecification, InventoryList, PriceGet };
use tower::{ Service, ServiceExt};

use crate::algoliarecords::ItemRecord;

mod algoliarecords;

const CONCURRENT_REQUESTS: usize = 12;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        let start = std::time::Instant::now();
        env_logger::Builder::from_default_env().format(move |buf, rec| {
            let t = start.elapsed().as_secs_f32();
            writeln!(buf, "{:.03} [{}] - {}", t, rec.level(),rec.args())
        }).init();
    })
}

pub async fn get_all_sku_ids_by_page(page: i32, client: &Client, sku_ids: &mut Vec<i32>) -> i32 {
    let url = "https://michaelvb.vtexcommercestable.com.br/api/catalog_system/pvt/sku/stockkeepingunitids?page={page}&pagesize=1000".to_string().replace("{page}", page.to_string().as_str());
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();

    // println!("response.status: {}", response.status());
    match response.status() {
        StatusCode::OK => {
            let response_text = response.text().await.unwrap();
            let ids = response_text.replace("[", "").replace("]", "");
            let iter = ids.split(",");
            let mut x = 0;
            // println!("ids: {:?}", ids);
            // let mut ids_response: Vec<i32> = Vec::new();
            for v in iter {
                sku_ids.push(v.parse::<i32>().unwrap());
                x = x + 1;
            }
            x
        },
        _ => {
            panic!("Status Code: [{:?}] Error: [{:#?}]", response.status(), response.text().await)
        },
    }
}

fn build_get_sku_urls(sku_ids: &Vec<i32>) -> Vec<String> {
    let url = "https://michaelvb.vtexcommercestable.com.br/api/catalog_system/pvt/sku/stockkeepingunitbyid/{skuId}?sc=1".to_string();
    let mut urls: Vec<String> = Vec::with_capacity(sku_ids.len());
    for sku_id in sku_ids {
        let url = url.replace("{skuId}", sku_id.to_string().as_str());
        urls.push(url);
    }
    debug!("sku urls.len(): {}", urls.len());
    urls
}

pub async fn get_sku_and_context(sku_id: &i32, client: &Client) -> SkuAndContext {
    let url = "https://michaelvb.vtexcommercestable.com.br/api/catalog_system/pvt/sku/stockkeepingunitbyid/{skuId}?sc=1".to_string().replace("{skuId}", sku_id.to_string().as_str());
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();

    // println!("response.status: {}", response.status());
    match response.status() {
        StatusCode::OK => {
            let response_text = response.text().await.unwrap();
            // println!("response_text: {:?}", response_text);
            let result: Result<SkuAndContext, serde_json::Error> = serde_json::from_str(&response_text);
            match result {
                Ok(sku_and_context) => {
                    // println!("sku_and_context: {:?}", sku_and_context);
                    return sku_and_context
                },
                Err(e) => {
                    // println!("deserialize product error: {:?}", e);
                    panic!("deserialize product error: {:?}", e)
                },
            }
        },
        _ => {
            panic!("Status Code: [{:?}] Error: [{:#?}]", response.status(), response.text().await)
        },
    }
}

fn build_get_price_urls(sku_ids: &Vec<i32>) -> Vec<String> {
    let url = "https://api.vtex.com/michaelvb/pricing/prices/{skuId}".to_string();
    let mut urls: Vec<String> = Vec::with_capacity(sku_ids.len());
    for sku_id in sku_ids {
        let url = url.replace("{skuId}", sku_id.to_string().as_str());
        urls.push(url);
    }
    debug!("price urls.len(): {}", urls.len());
    urls
}

fn build_price_for_algolia(vtex_price: &PriceGet) -> Price {
    let price = Price {
        value: vtex_price.base_price.unwrap().clone(),
        currency: "USD".to_string(),
        on_sales: false,
        discount_level: -100.00,
        discounted_value: 0.00,
    };
    price
}

pub async fn get_price(sku_id: &i32, client: &Client) -> Price {
    let url = "https://api.vtex.com/michaelvb/pricing/prices/{skuId}".to_string().replace("{skuId}", sku_id.to_string().as_str());
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();

    // println!("response.status: {}", response.status());
    match response.status() {
        StatusCode::OK => {
            let response_text = response.text().await.unwrap();
            // println!("response_text: {:?}", response_text);
            let result: Result<vtex::model::PriceGet, serde_json::Error> = serde_json::from_str(&response_text);
            match result {
                Ok(vtex_price) => {
                    // println!("vtex_price: {:?}", vtex_price);
                    let price = Price {
                        value: vtex_price.base_price.unwrap().clone(),
                        currency: "USD".to_string(),
                        on_sales: false,
                        discount_level: -100.00,
                        discounted_value: 0.00,
                    };
                    return price
                },
                Err(e) => {
                    // println!("deserialize product error: {:?}", e);
                    panic!("deserialize Price error: {:?}", e)
                },
            }
        },
        _ => {
            panic!("Status Code: [{:?}] Error: [{:#?}]", response.status(), response.text().await)
        },
    }
}

fn build_get_inventory_urls(sku_ids: &Vec<i32>) -> Vec<String> {
    let url = "https://michaelvb.vtexcommercestable.com.br/api/logistics/pvt/inventory/skus/{skuId}".to_string();
    let mut urls: Vec<String> = Vec::with_capacity(sku_ids.len());
    for sku_id in sku_ids {
        let url = url.replace("{skuId}", sku_id.to_string().as_str());
        urls.push(url);
    }
    debug!("inventory urls.len(): {}", urls.len());
    urls
}

fn get_inventory_for_algolia(vtex_inventory: &InventoryList) -> i32 {
    let mut quantity = 0;
    for balance in &vtex_inventory.balance {
        if balance.warehouse_id.eq("warehouse1") {
            quantity = balance.total_quantity;
        }
    }
    quantity
}

pub async fn get_inventory(sku_id: &i32, client: &Client) -> i32 {
    let url = "https://michaelvb.vtexcommercestable.com.br/api/logistics/pvt/inventory/skus/{skuId}".to_string().replace("{skuId}", sku_id.to_string().as_str());
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();

    // println!("response.status: {}", response.status());
    match response.status() {
        StatusCode::OK => {
            let response_text = response.text().await.unwrap();
            // println!("response_text: {:?}", response_text);
            let result: Result<vtex::model::InventoryList, serde_json::Error> = serde_json::from_str(&response_text);
            match result {
                Ok(vtex_inventory) => {
                    // println!("vtex_inventory: {:?}", vtex_inventory);
                    let mut quantity = 0;
                    for balance in vtex_inventory.balance {
                        if balance.warehouse_id.eq("wareshouse1") {
                            quantity = balance.total_quantity;
                        }
                    }
                    return quantity
                },
                Err(e) => {
                    // println!("deserialize product error: {:?}", e);
                    panic!("deserialize InventoryList error: {:?}", e)
                },
            }
        },
        _ => {
            panic!("Status Code: [{:?}] Error: [{:#?}]", response.status(), response.text().await)
        },
    }
}

fn get_hierarchical_categories(categories: &serde_json::Value, product_category_ids: &String) -> HierarchicalCategories {
    let cats = categories.as_object().unwrap();
    let lvl_keys: Vec<&str> = product_category_ids.split("/").collect();
    debug!("lvl_keys: {:?}", lvl_keys);
    // let mut cat_names: Vec<String> = Vec::new();
    // use a LinkedHashSet to preserve order
    let mut cat_names = HashMap::new();
    for cat in cats {
        // println!("cat: {:?}", cat.1.as_str().unwrap());
        cat_names.insert(cat.0.as_str(), cat.1.as_str().unwrap().to_string());
    }
    debug!("cat_names: {:?}", cat_names);
    // Pull the records in order off the back of the HashSet - this is how
    // VTEX provides the categories - lvl2, lvl1, lvl0
    let lvl0 = cat_names.get(lvl_keys[1]).unwrap().to_string();
    let lvl1 = lvl0.clone() + " > " + cat_names.get(lvl_keys[2]).unwrap().as_str();
    let lvl2 = lvl1.clone().to_owned() + " > " + cat_names.get(lvl_keys[3]).unwrap().as_str();
    HierarchicalCategories {
        lvl0: lvl0,
        lvl1: lvl1,
        lvl2: lvl2,
    }
}

fn get_list_categories(categories: &serde_json::Value, product_category_ids: &String) -> Vec<String> {
    let cats = categories.as_object().unwrap();
    let lvl_keys: Vec<&str> = product_category_ids.split("/").collect();
    debug!("lvl_keys: {:?}", lvl_keys);
    // let mut cat_names: Vec<String> = Vec::new();
    // use a LinkedHashSet to preserve order
    let mut cat_names = HashMap::new();
    for cat in cats {
        // println!("cat: {:?}", cat.1.as_str().unwrap());
        cat_names.insert(cat.0.as_str(), cat.1.as_str().unwrap().to_string());
    }
    debug!("cat_names: {:?}", cat_names);
    // Pull the records in order off the back of the HashSet - this is how
    // VTEX provides the categories - lvl2, lvl1, lvl0
    let lvl0 = cat_names.get(lvl_keys[1]).unwrap().to_string();
    let lvl1 = cat_names.get(lvl_keys[2]).unwrap().to_string();
    let lvl2 = cat_names.get(lvl_keys[3]).unwrap().to_string();

    vec![lvl0, lvl1, lvl2]
}

fn get_category_page_ids(categories: &serde_json::Value, product_category_ids: &String) -> Vec<String> {
    let cats = categories.as_object().unwrap();
    let lvl_keys: Vec<&str> = product_category_ids.split("/").collect();
    debug!("lvl_keys: {:?}", lvl_keys);
    // let mut cat_names: Vec<String> = Vec::new();
    // use a LinkedHashSet to preserve order
    let mut cat_names = HashMap::new();
    for cat in cats {
        // println!("cat: {:?}", cat.1.as_str().unwrap());
        cat_names.insert(cat.0.as_str(), cat.1.as_str().unwrap().to_string());
    }
    debug!("cat_names: {:?}", cat_names);
    // Pull the records in order off the back of the HashSet - this is how
    // VTEX provides the categories - lvl2, lvl1, lvl0
    let lvl0 = cat_names.get(lvl_keys[1]).unwrap().to_string();
    let lvl1 = lvl0 + " > " + cat_names.get(lvl_keys[2]).unwrap().as_str();
    let lvl2 = lvl1.clone().to_owned() + " > " + cat_names.get(lvl_keys[3]).unwrap().as_str();

    vec![lvl1, lvl2]
}

fn get_image_urls(images: &Option<Vec<Image>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for image in images.as_ref().unwrap() {
        result.push(image.image_url.clone());
    }
    result
}

fn get_color(sku_specs: &Option<Vec<SkuSpecification>>) -> Option<String> {
    let mut color: String = String::new();
    for spec in sku_specs.as_ref().unwrap() {
        if spec.field_name.eq("Color") {
            color = spec.field_values[0].clone();
        }
    }
    if !color.is_empty() {
        Some(color)
    } else {
        None
    }
}

fn get_size(sku_specs: &Option<Vec<SkuSpecification>>) -> Option<String> {
    let mut size: String = String::new();
    for spec in sku_specs.as_ref().unwrap() {
        if spec.field_name.eq("Size") {
            size = spec.field_values[0].clone();
        }
    }
    if !size.is_empty()  {
        Some(size)
    } else {
        None
    }
}

// fn get_variants(sku_ctx: &SkuAndContext) -> Vec<Variant> {
//     let variant = Variant {
//         sku_ref: sku_ctx.alternate_ids.ref_id.clone(),
//         in_stock: true,
//         abbreviated_color: get_color(&sku_ctx.sku_specifications),
//         abbreviated_size: get_size(&sku_ctx.sku_specifications),
//     };
//     vec![variant]
// }

async fn get_all_sku_ids(client: &Client) -> Vec<i32> {
    let start = Instant::now();
    info!("Start get_all_sku_ids()");
    // Get all the skus
    let sku_ids: &mut Vec<i32> = &mut Vec::new();
    let recs = &mut 1000;
    let page = &mut 1;

    while *recs == 1000 {
        *recs = get_all_sku_ids_by_page(page.clone(), &client, sku_ids).await;
        *page += 1;
    }
    let duration = start.elapsed();
    info!("Finished get_all_sku_ids: {} records in {:?}", sku_ids.len(), duration);
    sku_ids.to_vec()
}

async fn get_item_records(sku_ids: &Vec<i32>, client: &Client) ->HashMap<i32, SkuAndContext> {
    info!("Starting get_item_records()");
    // Build the urls
    let urls = build_get_sku_urls(&sku_ids);
    debug!("after call to build_get_sku_urls");
    let item_recs: Arc<Mutex<HashMap<i32, SkuAndContext>>> = Arc::new(Mutex::new(HashMap::new()));
    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = client
                    .get(url.clone()).send()
                    .await?;
                    
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
                    item_recs.insert(sku_ctx.id.clone(), sku_ctx.clone());
                    debug!("Got: {:?}", sku_ctx)
                },
                Err(e) => error!("Got an error: {}", e),
            }
        })
        .await;
    
    let ir = item_recs.lock().unwrap().clone();
    info!("finished get_item_records(): item_recs.len(): {:?}", ir.len());
    ir    
}

async fn get_price_records(sku_ids: &Vec<i32>, client: &Client) -> HashMap<i32, PriceGet> {
    info!("Starting get_price_records()");
    // build the urls
    let urls = build_get_price_urls(&sku_ids);
    let price_recs: Arc<Mutex<HashMap<i32, PriceGet>>> = Arc::new(Mutex::new(HashMap::new()));
    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = client
                    .get(url.clone()).send()
                    .await?;
                resp.json::<PriceGet>().await
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);
    bodies
        .for_each(|b| async {
            let price_recs = price_recs.clone();
            match b {
                Ok(b) => {
                    let price_list: PriceGet = b;
                    let mut price_recs = price_recs.lock().unwrap();
                    price_recs.insert(price_list.item_id.parse::<i32>().unwrap(), price_list.clone());
                    debug!("Got price_list.item_id: {:?}", price_list.item_id)
                },
                Err(e) => error!("Got an error: {}", e),
            }
        })
        .await;

    let pr = price_recs.lock().unwrap().clone();
    info!("finished get_price_records(): price_recs.len(): {:?}", pr.len());
    pr
}

async fn get_inventory_records(sku_ids: &Vec<i32>, client: &Client) -> HashMap<i32, InventoryList>{
    info!("Starting get_inventory_records()");
    // build the urls
    let urls = build_get_inventory_urls(&sku_ids);
    let inventory_recs: Arc<Mutex<HashMap<i32, InventoryList>>> = Arc::new(Mutex::new(HashMap::new()));
    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = client
                    .get(url.clone()).send()
                    .await?;
                resp.json::<InventoryList>().await
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);
    bodies
        .for_each(|b| async {
            let inventory_recs = inventory_recs.clone();
            match b {
                Ok(b) => {
                    let inventory_list: InventoryList = b;
                    let mut inventory_recs = inventory_recs.lock().unwrap();
                    inventory_recs.insert(inventory_list.sku_id.parse::<i32>().unwrap(), inventory_list.clone());
                    debug!("Got inventory_list.sku_id: {:?}", inventory_list.sku_id)
                },
                Err(e) => error!("Got an error: {}", e),
            }
        })
        .await;

    let invr = inventory_recs.lock().unwrap().clone();
    info!("finished get_inventory_records(): inventory_recs.len(): {:?}", invr.len());
    invr
}

// Struct to hold key attributes (Size, Color) and variants
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct ProductVariant {
    product_ref_id: String,
    available_sizes: Option<Vec<String>>,
    available_colors: Option<Vec<String>>,
    variants: Vec<Variant>,
}

fn build_product_variant_map(
    sku_ids: &Vec<i32>,
    item_records: &HashMap<i32, SkuAndContext>,
    inventory_records: &HashMap<i32, InventoryList>)
    -> HashMap<String, ProductVariant> {
        info!("Start build_product_variant_map()");
        let mut product_variants: HashMap<String, ProductVariant> = HashMap::with_capacity(sku_ids.len());
        // Loop through every sku and build the structure
        for sku_id in sku_ids {
            // Lookup key values
            let item_record = item_records.get(sku_id).unwrap();
            let sku_specs = &item_record.sku_specifications;
            let color = get_color(&sku_specs.clone());
            let size = get_size(sku_specs);
            let inventory_record = inventory_records.get(sku_id).unwrap();
            let in_stock: bool;
            if get_inventory_for_algolia(inventory_record) > 0 
            { 
                in_stock = true;
            } else { 
                in_stock = false;
            };
            let variant: Variant = Variant {
                 sku_ref: item_record.alternate_ids.ref_id.clone(),
                 abbreviated_color: color.clone(),
                 abbreviated_size: size.clone(),
                 in_stock: in_stock,
            };
            // First Product Variant or not
            if !product_variants.contains_key(&item_record.product_ref_id) {
                let mut available_colors = Vec::new();
                let mut available_sizes = Vec::new();
                let mut variants = Vec::new();
                if color.is_some() { available_colors.push(color.unwrap().clone()) };
                // available_colors.push(color.clone());
                // let option_avail_colors: Option<Vec<String>> = 
                //     available_colors.m
                if size.is_some() { available_sizes.push(size.unwrap().clone()) };
                // available_sizes.push(size.clone());
                variants.push(variant);
                let prod_variant: ProductVariant = ProductVariant {
                    product_ref_id: item_record.product_ref_id.clone(),
                    available_colors: Some(available_colors),
                    available_sizes: Some(available_sizes),
                    variants: variants,
                };
                product_variants.insert(item_record.product_ref_id.clone(), prod_variant);
                debug!("inserted new product_variant: {}", item_record.product_ref_id);
            } else {
                let product_variant = product_variants.get_mut(&item_record.product_ref_id).unwrap();
                if product_variant.available_colors.is_some() {
                    if color.is_some() {
                        //product_variant.available_colors.as_ref().unwrap().push(color.unwrap());
                        if !product_variant.available_colors.as_ref().unwrap().contains(color.as_ref().unwrap()) {
                            product_variant.available_colors.as_mut().unwrap().push(color.unwrap().clone());
                        }
                    }
                }
                if product_variant.available_sizes.is_some() {
                    if size.is_some() {
                        // product_variant.available_sizes.unwrap().push(size.unwrap());
                        if !product_variant.available_sizes.as_ref().unwrap().contains(size.as_ref().unwrap()) {
                            product_variant.available_sizes.as_mut().unwrap().push(size.unwrap());
                        }
                    }
                }
                // let v: &mut Vec<Variant> = product_variant.variants.as_mut();
                // info!("product_variant value: {:?}", v);
                // v.push(variant);
                product_variant.variants.push(variant);
                debug!("updated product_variant: {}", product_variant.product_ref_id);
            }
        }
        info!("Finished build_product_variant_map(): {} records", product_variants.len());
        product_variants
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct SkusInactive {
//     pub sku_id: i32,
// }

// fn gen_skus_missing_images_file(sku_ids: &Vec<i32>, item_records: &HashMap<i32, SkuAndContext>) {
//     let out_path = "data/skus_missing_images.csv";
//     let mut writer = csv::Writer::from_path(out_path).unwrap();
//     let mut x = 0;
//     for sku_id in sku_ids {
//         // Lookup key values
//         let item_record = item_records.get(sku_id).unwrap();
//         if get_image_urls(&item_record.images).is_empty() {
//             writer.serialize(SkusInactive { sku_id: sku_id.clone()}).unwrap();
//             x = x + 1;
//         }
//     }
//     // Flush the records
//     writer.flush().unwrap();
//     info!("skus with missing images: {}", x);
// }

fn generate_review() -> Review {
    let mut rng = rand::thread_rng();
    let bay_avg = format!("{:.1$}", rng.gen_range(0.0..5.0), 2).parse::<f32>().unwrap();
    // let rating = format!("{:.1$}", bay_avg, 0);
    Review {
        rating: bay_avg.round() as i32,
        count: rng.gen_range(0..200),
        bayesian_avg: bay_avg,
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {

    info!("Start of run()");
    dotenv::dotenv().expect("Failed to read .env file");

    let vtex_api_key = env::var("VTEX_API_APPKEY").expect("Failed to parse VTEX_API_APPKEY in .env");
    let vtex_api_apptoken = env::var("VTEX_API_APPTOKEN").expect("Failed to parse VTEX_API_APPTOKEN in .env");
    
    // Setup the HTTP client
    let mut headers = header::HeaderMap::new();
    headers.insert("X-VTEX-API-AppKey", header::HeaderValue::from_str(&vtex_api_key)?);
    headers.insert("X-VTEX-API-AppToken", header::HeaderValue::from_str(&vtex_api_apptoken)?);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build()?;

    // Get all the sku_ids in VTEX
    let sku_ids = get_all_sku_ids(&client).await;
    // // Get SkuAndContext records
    // get_item_records(&sku_ids, &client).await;
    // // Get Price records
    // get_price_records(&sku_ids, &client).await;
    // // Get Inventory records
    // get_inventory_records(&sku_ids, &client).await;

    //Run concurrently
    let ir = get_item_records(&sku_ids, &client);
    // Get Price records
    let pr = get_price_records(&sku_ids, &client);
    // Get Inventory records
    let invr = get_inventory_records(&sku_ids, &client);
    // join! all the futures to run concurrently
    let (ir, pr, invr) = join!(ir, pr, invr);
    debug!("inventory map: {:?}", invr);
    // Generate the list of sku's with missing images
    // gen_skus_missing_images_file(&sku_ids, &ir);
    
    let mut algolia_recs: Vec<ItemRecord> = Vec::with_capacity(sku_ids.len());
    let path = "data/algolia_records.json";
    let out_file = File::create(path)?;
    let mut buf_wtr = BufWriter::new(out_file);

    // Need to add the builds for the variants, sizes and colors
    let product_variants = build_product_variant_map(&sku_ids, &ir, &invr);
    // Build the Algolia Records
    info!("Starting algolia record build");
    for sku_id in sku_ids {
        let sku_ctx = ir.get(&sku_id).unwrap();
        let price_get = pr.get(&sku_id).unwrap();
        let inventory_list = invr.get(&sku_id).unwrap();
        debug!("product_variants: {:?}", product_variants);
        debug!("Retrieving product variant: {}", sku_ctx.product_ref_id);
        let product_variant = product_variants.get(&sku_ctx.product_ref_id.clone()).unwrap();

        // Build the Algolia Record
        let algolia_record = ItemRecord {
            sku_id: sku_ctx.id.clone(),
            sku_ref: sku_ctx.alternate_ids.ref_id.clone(),
            product_id: sku_ctx.product_id.clone(),
            parent_ref: sku_ctx.product_ref_id.clone(),
            name: sku_ctx.product_name.clone(),
            description: sku_ctx.product_description.clone(),
            slug: sku_ctx.detail_url.clone(),
            brand: sku_ctx.brand_name.clone(),
            hierarchical_categories: get_hierarchical_categories(&sku_ctx.product_categories, &sku_ctx.product_category_ids),
            list_categories: get_list_categories(&sku_ctx.product_categories, &sku_ctx.product_category_ids),
            category_page_id: get_category_page_ids(&sku_ctx.product_categories, &sku_ctx.product_category_ids),
            image_urls: get_image_urls(&sku_ctx.images),
            image_blurred: None,
            reviews: Some(generate_review()),
            color: get_color(&sku_ctx.sku_specifications),
            available_colors: product_variant.available_colors.clone(),
            size: get_size(&sku_ctx.sku_specifications),
            available_sizes: product_variant.available_sizes.clone(),
            variants: product_variant.variants.clone(),
            price: build_price_for_algolia(price_get),
            units_in_stock: get_inventory_for_algolia(inventory_list),
            created_at: None,
            updated_at: None,
            related_products: None,
            product_type: None,
            object_id: sku_ctx.alternate_ids.ref_id.clone(),
        };
        algolia_recs.push(algolia_record);
    }
    info!("Finished building algolia records: {}", algolia_recs.len());
    let result = serde_json::to_string_pretty(&algolia_recs)?;
    buf_wtr.write_all(result.as_bytes())?;
    buf_wtr.flush()?;
    info!("Finished writing algolia records to file: {}", path);
  
    Ok(())
}

pub async fn test_rate_limit() -> Result<(), Box<dyn Error>> {
    info!("Start of run()");
    dotenv::dotenv().expect("Failed to read .env file");

    let vtex_api_key = env::var("VTEX_API_APPKEY").expect("Failed to parse VTEX_API_APPKEY in .env");
    let vtex_api_apptoken = env::var("VTEX_API_APPTOKEN").expect("Failed to parse VTEX_API_APPTOKEN in .env");
    
    // Setup the HTTP client
    let mut headers = header::HeaderMap::new();
    headers.insert("X-VTEX-API-AppKey", header::HeaderValue::from_str(&vtex_api_key)?);
    headers.insert("X-VTEX-API-AppToken", header::HeaderValue::from_str(&vtex_api_apptoken)?);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build()?;

    // Get all the sku_ids in VTEX
    let sku_ids = get_all_sku_ids(&client).await;

    let urls = build_get_sku_urls(&sku_ids);
    debug!("after call to build_get_sku_urls");

    let mut svc = tower::ServiceBuilder::new()
        .rate_limit(160, Duration::new(1, 0)) // 100 requests every 10 seconds
        .service(tower::service_fn(move |req| client.execute(req)));

    for url in urls {
        let req = Request::new(Method::GET, Url::parse(&url).unwrap());
        let resp = svc.ready().await?.call(req).await?;
        // info!("resp.status(): {:?}", resp);
    }

    // let item_recs: Arc<Mutex<HashMap<i32, SkuAndContext>>> = Arc::new(Mutex::new(HashMap::new()));
    // let bodies = stream::iter(urls)
    //     .map(|url| {
    //         let svc = &mut svc;
    //         async move {
    //             // let u = Url::parse(&url).unwrap();
    //             let req = Request::new(Method::GET, Url::parse(&url).unwrap());
    //             let resp = svc.ready().await?.call(req).await?;
    //             // let resp = client
    //             //     .get(url.clone()).send()
    //             //     .await?;
                    
    //             // let sctx: SkuAndContext = resp.json().await?;
    //             debug!("end of async move - url: {}", url);
    //             // resp.text().await
    //             resp.json::<SkuAndContext>().await
    //         }
    //     })
    //     .buffer_unordered(CONCURRENT_REQUESTS);
    // bodies
    //     .for_each(|b| async {
    //         let item_recs = item_recs.clone();
    //         match b {
    //             Ok(b) => {
    //                 // let result: Result<SkuAndContext, serde_json::Error> = serde_json::from_str(b).unwrap();
    //                 let sku_ctx: SkuAndContext = b;
    //                 let mut item_recs = item_recs.lock().unwrap();
    //                 item_recs.insert(sku_ctx.id.clone(), sku_ctx.clone());
    //                 debug!("Got: {:?}", sku_ctx)
    //             },
    //             Err(e) => error!("Got an error: {}", e),
    //         }
    //     })
    //     .await;
    
    // let ir = item_recs.lock().unwrap().clone();
    info!("finished get_item_records()");


    Ok(())

}