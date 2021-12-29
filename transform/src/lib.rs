use dotenv;
use std::env;
use std::error::Error;
use reqwest::{header};
use clap::{Arg, App, crate_version};

mod utils;
mod productspecsdesc;
mod prodspecsdefine;
mod fieldvalues;
mod csvrecords;
mod products;
mod skus;
mod genskudefiningattrvalues;
mod skuspecassign;
mod skufiles;
mod prices;
mod inventory;
mod geninactiveskufiles;

struct Command {
    object: String,
    // input_file: String,
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
            .help("The object you are loading. Valid values: productspecsdesc, productspecsdefine, fieldvalues, product")
            .takes_value(true))
        // .arg(Arg::with_name("FILE")
        //     .required(true)
        //     .short("f")
        //     .long("file")
        //     .value_name("FILE")
        //     .help("Sets the input file to use")
        //     .takes_value(true))
        .get_matches();

        let vtex_object = matches.value_of("OBJECT").expect("-o <OBJECT> must be set (example: Category, Brand, etc.");
        println!("vtex_object: {}", vtex_object);
        // let vtex_object1 = match vtex_object {
        //     Some(vtex_object1) => { vtex_object1 }
        //     None => { return Err("-o <OBJECT> must be set (example: category, brand, etc.)") }
        // };
        // let input_file = matches.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/categories.csv");
        // println!("input_file: {}", input_file);

        Command { object: vtex_object.to_string() }
    }

    fn validate_vtex_object(v: String) -> Result<(), String> {
        println!("command: {}", v);
        let valid_objects = ["productspecsdesc", "productspecsdefine", "fieldvalues", "product", "sku", "genskudefiningattrvalues", "skuspecassign", "skufile", "price", "inventory", "geninactiveskufiles"];
        if valid_objects.contains(&v.as_str()) { return Ok(()); }
        Err(String::from("Must set a valid VTEX object: productspecsdesc, productspecsdefine, fieldvalues, product, sku, genskudefiningattrvalues, skuspecassign, skufile, price, inventory, geninactiveskufiles"))
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {

    let cmd = Command::get_command();
    dotenv::dotenv().expect("Failed to read .env file");
    let vtex_api_key = env::var("VTEX_API_APPKEY").expect("Failed to parse VTEX_API_APPKEY in .env");
    let vtex_api_apptoken = env::var("VTEX_API_APPTOKEN").expect("Failed to parse VTEX_API_APPTOKEN in .env");
    let specs_group_url = env::var("LIST_SPECIFICATIONS_GROUP_URL").expect("Failed to parse IST_SPECIFICATIONS_GROUP_URL in .env");
    let category_tree_url = env::var("CATEGORY_TREE_URL").expect("Failed to parse CATEGORY_TREE_URL in .env");
    let spec_fields_for_category_url = env::var("LIST_SPEC_FIELDS_FOR_CATEGORY_URL").expect("Failed to parse LIST_SPEC_FIELDS_FOR_CATEGORY_URL");
    let brand_list_url = env::var("BRAND_LIST_URL").expect("Failed to parse BRAND_LIST_URL");
    let field_values_for_field_url = env::var("FIELD_VALUES_LIST_URL").expect("Failed to parse FIELD_VALUES_LIST_URL");
    // let group_url = env::var("GROUP_URL").expect("Failed to parse GROUP_URL in .env");
    // let specification_url = env::var("SPECIFICATION_URL").expect("Failed to parse SPECIFICATION_URL in .env");
    
    // Setup the HTTP client
    let mut headers = header::HeaderMap::new();
    headers.insert("X-VTEX-API-AppKey", header::HeaderValue::from_str(&vtex_api_key)?);
    headers.insert("X-VTEX-API-AppToken", header::HeaderValue::from_str(&vtex_api_apptoken)?);
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    if cmd.object.eq("productspecsdesc") {
        // Create Product Specifications csv
        println!("before call to load_categories(): {:?}", env::current_dir()?);
        // let result = categories::load_categories("data/DeptCatalog-sorted-subset.csv".to_string(), &client, category_url).await?;
        let result = productspecsdesc::build_product_specs_file(&client, specs_group_url, category_tree_url)?;
        println!("after call to load_categories(): {:?}", result);
        println!("result: {:?}", result);
    
    } else if cmd.object.eq("productspecsdefine") {
        // Load Defining Product Specs
        println!("before call to load_brands(): {:?}", env::current_dir()?);
        // let result = brands::load_brands("data/brands.csv".to_string(), &client, brand_url).await?;
        let result = prodspecsdefine::build_product_specs_file(&client, specs_group_url, category_tree_url)?;
        // println!("after call to load_brands(): {:?}", result);
        println!("result: {:?}", result);
    
    } else if cmd.object.eq("fieldvalues") {
        // Load fieldvalues
        let result = fieldvalues::build_field_values_file(&client, category_tree_url, spec_fields_for_category_url)?;
        println!("result: {:?}", result);
    } else if cmd.object.eq("product") {
        // Load products
        let result = products::build_product_file(&client, category_tree_url, brand_list_url)?;
        println!("result: {:?}", result);
    } else if cmd.object.eq("sku") {
        // Load skus
        let result = skus::build_sku_file()?;
        println!("result: {:?}", result);
    } else if cmd.object.eq("skuspecassign") {
        // Load sku spec assignments
        let result = skuspecassign::build_sku_spec_assign_file(&client, category_tree_url, spec_fields_for_category_url, field_values_for_field_url)?;
        println!("result: {:?}", result);
    } else if cmd.object.eq("skufile") {
        // Load sku files
        let result = skufiles::build_sku_file()?;
        println!("result: {:?}", result);
    } else if cmd.object.eq("price") {
        // Load prices
        let result = prices::build_price_file()?;
        println!("result: {:?}", result);
    } else if cmd.object.eq("inventory") {
        // Load inventory
        let result = inventory::build_inventory_file()?;
        println!("result: {:?}", result);
    } else if cmd.object.eq("genskudefiningattrvalues") {
        // Generate SKUDefiningAttributeValue-sorted-subset.csv file
        let result = genskudefiningattrvalues::gen_attr_value_subset()?;
        println!("result: {:?}", result);
    } else if cmd.object.eq("geninactiveskufiles") {
        // Generate inactive sku files
        let result = geninactiveskufiles::gen_inactive_sku_files()?;
        println!("result: {:?}", result);
    } else {
        println!("Did not enter a valid object - category or brand");
    }

    Ok(())
}

