use dotenv;
use log::*;
use std::{env, time::Duration};
use std::error::Error;
use std::io::Write;
use std::sync::Once;
use reqwest::header;
use clap::{Arg, App, crate_version};

mod brands;
mod categories;
mod groups;
mod specifications;
mod fieldvalues;
mod products;
mod skus;
mod productspecifications;
mod skuspecassignment;
mod skufiles;
mod csvrecords;
mod prices;
mod inventory;

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

struct Command {
    object: String,
    input_file: String,
}

impl Command {
    fn get_command() -> Command {
        // Retrieve variables from the command line
        let matches = App::new("VTEX Dataloader")
        .version(crate_version!())
        .author("VTEX")
        .about("Command line interface to batch load data into VTEX")
        .arg(Arg::with_name("OBJECT")
            .required(true)
            .validator(Command::validate_vtex_object)
            .short("o")
            .long("object")
            .value_name("VTEX OBJECT")
            .help("The object you are loading. Valid values: category, brand, group, specification, fieldvalue, product, sku")
            .takes_value(true))
        .arg(Arg::with_name("FILE")
            .required(true)
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("Sets the input file to use")
            .takes_value(true))
        .get_matches();

        let vtex_object = matches.value_of("OBJECT").expect("-o <OBJECT> must be set (example: Category, Brand, etc.");
        debug!("vtex_object: {}", vtex_object);
        // let vtex_object1 = match vtex_object {
        //     Some(vtex_object1) => { vtex_object1 }
        //     None => { return Err("-o <OBJECT> must be set (example: category, brand, etc.)") }
        // };
        let input_file = matches.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/categories.csv");
        debug!("input_file: {}", input_file);

        Command { object: vtex_object.to_string(), input_file: input_file.to_string() }
    }

    fn validate_vtex_object(v: String) -> Result<(), String> {
        let valid_objects = ["category", "brand", "group", "specification", "fieldvalue", "product", "sku", "productspecification", "skuspecassignment", "skufile", "price", "inventory"];
        if valid_objects.contains(&v.as_str()) { return Ok(()); }
        Err(String::from("Must set a valid VTEX object: category, brand, group, specification, fieldvalue, product, sku, productspecification, skuspecassignment, skufile, price, inventory"))
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {

    let cmd = Command::get_command();
    dotenv::dotenv().expect("Failed to read .env file");
    let account_name = env::var("ACCOUNT_NAME").expect("Failed to parse ACCOUNT_NAME");
    let environment = env::var("ENVIRONMENT").expect("Failed to parse ENVIRONMENT");
    let vtex_api_key = env::var("VTEX_API_APPKEY").expect("Failed to parse VTEX_API_APPKEY in .env");
    let vtex_api_apptoken = env::var("VTEX_API_APPTOKEN").expect("Failed to parse VTEX_API_APPTOKEN in .env");
    let category_url = env::var("CATEGORY_URL").expect("Failed to parse CATEGORY_URL in .env");
    let brand_url = env::var("BRAND_URL").expect("Failed to parse BRAND_URL in .env");
    let group_url = env::var("GROUP_URL").expect("Failed to parse GROUP_URL in .env");
    let specification_url = env::var("SPECIFICATION_URL").expect("Failed to parse SPECIFICATION_URL in .env");
    let fieldvalues_url = env::var("FIELDVALUES_URL").expect("Failed to parse FIELDVALUES_URL in .env");
    let products_url = env::var("PRODUCTS_URL").expect("Failed to parse PRODUCTS_URL in .env");
    let sku_url = env::var("SKU_URL").expect("Failed to parse SKU_URL in .env");
    let prod_spec_url = env::var("PRODUCT_SPECIFICATION_URL").expect("Failed to parse PRODUCT_SPECIFICATION_URL in .env");
    let sku_spec_url = env::var("SKU_SPECIFICATION_URL").expect("Failed to parse SKU_SPECIFICATION_URL in .env");
    let sku_file_url = env::var("SKU_FILE_URL").expect("Failed to parse SKU_FILE_URL in .env");
    let price_url = env::var("PRICE_URL").expect("Failed to parse PRICE_URL in .env");
    let inventory_url = env::var("INVENTORY_URL").expect("Failed to parse INVENTORY_URL in .env");

    // Setup the HTTP client
    let mut headers = header::HeaderMap::new();
    headers.insert("X-VTEX-API-AppKey", header::HeaderValue::from_str(&vtex_api_key)?);
    headers.insert("X-VTEX-API-AppToken", header::HeaderValue::from_str(&vtex_api_apptoken)?);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .default_headers(headers)
        .build()?;

    // match cmd.object.as_str() {
    //     "category" => { println!("category"); }
    //     _ => { return Err("missed")}
    // }

    if cmd.object.eq("category") {
        // Load Categories
        // println!("before call to load_categories(): {:?}", env::current_dir()?);
        // let result = categories::load_categories("data/DeptCatalog-sorted-subset.csv".to_string(), &client, category_url).await?;
        categories::load_categories(cmd.input_file.to_string(), &client, category_url).await?;
        // println!("after call to load_categories(): {:?}", result);
        info!("finished loading categories");
    
    } else if cmd.object.eq("brand") {
        // Load Brands
        // println!("before call to load_brands(): {:?}", env::current_dir()?);
        // let result = brands::load_brands("data/brands.csv".to_string(), &client, brand_url).await?;
        brands::load_brands(cmd.input_file.to_string(), &client, brand_url).await?;
        // println!("after call to load_brands(): {:?}", result);
        info!("finished loading brands");
    
    } else if cmd.object.eq("group") {
        // Load groups
        groups::load_groups(cmd.input_file.to_string(), &client, group_url).await?;
        info!("finished loading groups");
    } else if cmd.object.eq("specification") {
        // Load specifications
        specifications::load_specifications(cmd.input_file.to_string(), &client, specification_url).await?;
        info!("finished loading specifications");
    } else if cmd.object.eq("fieldvalue") {
        // Load field values
        fieldvalues::load_field_values(cmd.input_file.to_string(), &client, fieldvalues_url).await?;
        info!("finished loading fieldvalues");
    } else if cmd.object.eq("product") {
        // Load products
        products::load_products(cmd.input_file.to_string(), &client, products_url).await?;
        info!("finished loading products");
    } else if cmd.object.eq("sku") {
        // Load skus
        skus::load_skus(cmd.input_file.to_string(), &client, sku_url).await?;
        info!("finished loading skus");
    } else if cmd.object.eq("productspecification") {
        // Load product specs
        productspecifications::load_product_specs(cmd.input_file.to_string(), &client, prod_spec_url).await?;
        info!("finished loading product specifications");
    } else if cmd.object.eq("skuspecassignment") {
        // Load sku spec assignments
        skuspecassignment::load_sku_specs(cmd.input_file.to_string(), &client, sku_spec_url).await?;
        info!("finished loading sku spec assignments");
    } else if cmd.object.eq("skufile") {
        // Load sku files
        skufiles::load_sku_files(cmd.input_file.to_string(), &client, sku_file_url).await?;
        info!("finished loading sku files");
    } else if cmd.object.eq("price") {
        // Load sku files
        prices::load_prices(cmd.input_file.to_string(), &client, price_url).await?;
        info!("finished loading prices");
    } else if cmd.object.eq("inventory") {
        // Load sku files
        inventory::load_inventory_concurrent(cmd.input_file.to_string(), &client, inventory_url).await?;
        info!("finished loading inventory");
    } else {
        info!("Did not enter a valid object - category or brand");
    }

    Ok(())
}
