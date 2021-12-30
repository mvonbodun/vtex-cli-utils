use algoliarecords::{HierarchicalCategories, Variant, Price};
use dotenv;
use log::*;
use std::{error::Error, time::{Duration, Instant}, env, sync::Once, collections::HashMap, ops::Index};
use std::io::Write;
use std::sync::{Arc, Mutex};
use futures::{join, stream, StreamExt};

use reqwest::{header, StatusCode, Client};
use vtex::model::{SkuAndContext, Image, SkuSpecification };

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

fn build_get_sku_urls(sku_ids: &mut Vec<i32>) -> Vec<String> {
    let url = "https://michaelvb.vtexcommercestable.com.br/api/catalog_system/pvt/sku/stockkeepingunitbyid/{skuId}?sc=1".to_string();
    let mut urls: Vec<String> = Vec::with_capacity(sku_ids.len());
    for sku_id in sku_ids {
        let url = url.replace("{skuId}", sku_id.to_string().as_str());
        urls.push(url);
    }
    debug!("urls.len(): {}", urls.len());
    urls
    //.replace("{skuId}", sku_id.to_string().as_str());

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

fn get_hierarchical_categories(categories: &serde_json::Value) -> HierarchicalCategories {
    let cats = categories.as_object().unwrap();
    let mut cat_names: Vec<String> = Vec::new();
    for cat in cats {
        // println!("cat: {:?}", cat.1.as_str().unwrap());
        cat_names.push(cat.1.as_str().unwrap().to_string());
    }
    HierarchicalCategories {
        lvl0: cat_names.get(0).unwrap().as_str().to_string(),
        lvl1: cat_names.get(1).unwrap().as_str().to_string(),
        lvl2: cat_names.get(2).unwrap().as_str().to_string(),
    }
}

fn get_list_categories(categories: &serde_json::Value) -> Vec<String> {
    let cats = categories.as_object().unwrap();
    let mut cat_names: Vec<String> = Vec::new();
    for cat in cats {
        // println!("cat: {:?}", cat.1.as_str().unwrap());
        cat_names.push(cat.1.as_str().unwrap().to_string());
    }
    let lvl0 = cat_names.get(0).unwrap().as_str().to_string();
    let lvl1 = cat_names.get(1).unwrap().as_str().to_string();
    let lvl2 = cat_names.get(2).unwrap().as_str().to_string();
    vec![lvl0, lvl1, lvl2]
}

fn get_category_page_ids(categories: &serde_json::Value) -> Vec<String> {
    let cats = categories.as_object().unwrap();
    let mut cat_names: Vec<String> = Vec::new();
    for cat in cats {
        // println!("cat: {:?}", cat.1.as_str().unwrap());
        cat_names.push(cat.1.as_str().unwrap().to_string());
    }
    let lvl1 = cat_names.get(0).unwrap().as_str().to_string() + " > " + cat_names.get(1).unwrap().as_str();
    let lvl2 = lvl1.clone().to_owned() + " > " + cat_names.get(2).unwrap().as_str();
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

fn get_variants(sku_ctx: &SkuAndContext) -> Vec<Variant> {
    let variant = Variant {
        sku_ref: sku_ctx.alternate_ids.ref_id.clone(),
        in_stock: true,
        abbreviated_color: get_color(&sku_ctx.sku_specifications),
        abbreviated_size: get_size(&sku_ctx.sku_specifications),
    };
    vec![variant]
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

    // Get all the skus
    let sku_ids: &mut Vec<i32> = &mut Vec::new();
    let recs = &mut 1000;
    let page = &mut 1;
    let start = Instant::now();
    while *recs == 1000 {
        *recs = get_all_sku_ids_by_page(page.clone(), &client, sku_ids).await;
        *page += 1;
    }
    let duration = start.elapsed();
    println!("Retrieved Sku List: {} records in {:?}", sku_ids.len(), duration);

    // Build the urls
    let urls = build_get_sku_urls(sku_ids);
    debug!("after call to build_get_sku_urls");
    let algolia_recs: Arc<Mutex<HashMap<i32, SkuAndContext>>> = Arc::new(Mutex::new(HashMap::new()));
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
            let algolia_recs = algolia_recs.clone();
            match b {
                Ok(b) => {
                    // let result: Result<SkuAndContext, serde_json::Error> = serde_json::from_str(b).unwrap();
                    let sku_ctx: SkuAndContext = b;
                    let mut algolia_recs = algolia_recs.lock().unwrap();
                    algolia_recs.insert(sku_ctx.id.clone(), sku_ctx.clone());
                    debug!("Got {:?} json", sku_ctx)
                },
                Err(e) => error!("Got an error: {}", e),
            }
        })
        .await;
    {
        debug!("finished sku_loop: algolia_recs.len(): {:?}", algolia_recs.lock().unwrap());
    }
    // // Loop through the skus to build each algolia record
    // let loop_start = Instant::now();
    // for sku in sku_ids {
    //     let start = Instant::now();
    //     let sku_ctx = get_sku_and_context(&sku, &client);
    //     let p = get_price(&sku, &client);
    //     let i = get_inventory(&sku, &client);
    //     let (sku_ctx, p, i) = join!(sku_ctx, p, i);
    //     let algolia_record = ItemRecord {
    //         sku_id: sku_ctx.id.clone(),
    //         sku_ref: sku_ctx.alternate_ids.ref_id.clone(),
    //         product_id: sku_ctx.product_id.clone(),
    //         parent_ref: sku_ctx.product_ref_id.clone(),
    //         name: sku_ctx.product_name.clone(),
    //         description: sku_ctx.product_description.clone(),
    //         slug: sku_ctx.detail_url.clone(),
    //         brand: sku_ctx.brand_name.clone(),
    //         hierarchical_categories: get_hierarchical_categories(&sku_ctx.product_categories),
    //         list_categories: get_list_categories(&sku_ctx.product_categories),
    //         category_page_id: get_category_page_ids(&sku_ctx.product_categories),
    //         image_urls: get_image_urls(&sku_ctx.images),
    //         image_blurred: None,
    //         reviews: None,
    //         color: get_color(&sku_ctx.sku_specifications),
    //         available_colors: None,
    //         size: get_size(&sku_ctx.sku_specifications),
    //         available_sizes: None,
    //         variants: get_variants(&sku_ctx),
    //         price: p,
    //         units_in_stock: i,
    //         created_at: None,
    //         updated_at: None,
    //         related_products: None,
    //         product_type: None,
    //         object_id: sku_ctx.alternate_ids.ref_id.clone(),
    //     };
    //     let duration = start.elapsed();
    //     // println!("algolia record: {:?}", algolia_record);
    //     info!("processed: sku_id: {}, sku_ref: {}, in {:?}", algolia_record.sku_id, algolia_record.sku_ref, duration);
    // }
    // let loop_end = loop_start.elapsed();
    // info!("finished building algolia records in {:?}", loop_end);
   
    Ok(())
}