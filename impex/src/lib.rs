use dotenv;
use log::*;
use std::num::{NonZeroU32};
use std::{env, time::Duration};
use std::error::Error;
use std::io::Write;
use std::sync::Once;
use reqwest::header;
use clap::{Arg, App, crate_version, arg_enum, SubCommand};

mod brands;
mod categories;
mod specificationgroups;
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

#[derive(Debug)]
struct Command {
    object: String,
    action: String,
    input_file: String,
    prod_spec_assign_file: String,
    sku_spec_allowed_values_file: String,
    product_file: String,
    concurrency: usize,
    rate_limit: NonZeroU32,
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum CategoryActions {
        import
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum BrandActions {
        import
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum SpecificationGroupActions {
        import
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum SpecificationActions {
        import,
        genproductspecsfile,
        genskuspecsfile,
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum ProductActions {
        import
    }
}

impl Command {
    fn get_command() -> Command {
        // Retrieve variables from the command line
        let matches = App::new("VTEX Dataloader")
        .version(crate_version!())
        .author("VTEX")
        .about("Command line interface to import / export data into VTEX")
        .subcommand(SubCommand::with_name("category")
            .about("actions on the category into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&CategoryActions::variants())
                .short("a")
                .long("action")
                .value_name("ACTION")
                .help("The action to perform on the VTEX Object - import, export")
                .takes_value(true))
            .arg(Arg::with_name("FILE")
                .required(true)
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets the input or output file to read or write to.")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("brand")
            .about("actions on the brand into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&BrandActions::variants())
                .short("a")
                .long("action")
                .value_name("ACTION")
                .help("The action to perform on the VTEX Object - import, export")
                .takes_value(true))
            .arg(Arg::with_name("FILE")
                .required(true)
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets the input or output file to read or write to.")
                .takes_value(true))
            .arg(Arg::with_name("CONCURRENCY")
                .short("c")
                .long("concurrency")
                .value_name("CONCURRENCY")
                .help("Sets the concurrency value - default is 1")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("specificationgroup")
            .about("actions on the specification group into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&SpecificationGroupActions::variants())
                .short("a")
                .long("action")
                .value_name("ACTION")
                .help("The action to perform on the VTEX Object - import, export")
                .takes_value(true))
            .arg(Arg::with_name("FILE")
                .required(true)
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets the input or output file to read or write to.")
                .takes_value(true))
            .arg(Arg::with_name("CONCURRENCY")
                .short("c")
                .long("concurrency")
                .value_name("CONCURRENCY")
                .help("Sets the concurrency value - default is 1")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("specification")
            .about("actions for operating on a specification into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&SpecificationActions::variants())
                .short("a")
                .long("action")
                .value_name("ACTION")
                .help("The action to perform on the VTEX Object - import, export")
                .takes_value(true))
            .arg(Arg::with_name("FILE")
                .required(true)
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets the input or output file to read or write to.")
                .takes_value(true))
            .arg(Arg::with_name("PRODUCT_SPEC_ASSIGNMENTS_FILE")
                .required(false)
                .long("prod_spec_assigns_file")
                .value_name("PRODUCT_SPEC_ASSIGNMENTS_FILE")
                .help("Sets the Product Specification Assignments file")
                .takes_value(true))
            .arg(Arg::with_name("SKU_SPEC_ALLOWED_VALUES_FILE")
                .required(false)
                .long("sku_spec_allowed_values_file")
                .value_name("SKU_SPEC_ALLOWED_VALUES_FILE")
                .help("Sets the SKU Specification Allowed Values file")
                .takes_value(true))
            .arg(Arg::with_name("PRODUCT_FILE")
                .required(false)
                .long("product_file")
                .value_name("PRODUCT_FILE")
                .help("Sets the Product file")
                .takes_value(true))
            .arg(Arg::with_name("CONCURRENCY")
                .short("c")
                .long("concurrency")
                .value_name("CONCURRENCY")
                .help("Sets the concurrency value - default is 1")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("product")
            .about("actions on the product into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&ProductActions::variants())
                .short("a")
                .long("action")
                .value_name("ACTION")
                .help("The action to perform on the VTEX Object - import, export")
                .takes_value(true))
            .arg(Arg::with_name("FILE")
                .required(true)
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets the input or output file to read or write to.")
                .takes_value(true))
            .arg(Arg::with_name("CONCURRENCY")
                .short("c")
                .long("concurrency")
                .value_name("CONCURRENCY")
                .help("Sets the concurrency value - default is 1")
                .takes_value(true))
            .arg(Arg::with_name("RATELIMIT")
                .short("r")
                .long("rate_limit")
                .value_name("RATELIMIT")
                .help("Sets the rate limit value (how many calls per second) - default is 200")
                .takes_value(true))
        )
        .get_matches();
    
        // .arg(Arg::with_name("OBJECT")
        //     .required(true)
        //     .validator(Command::validate_vtex_object)
        //     .short("o")
        //     .long("object")
        //     .value_name("VTEX OBJECT")
        //     .help("The object you are loading. Valid values: category, brand, group, specification, fieldvalue, product, sku")
        //     .takes_value(true))
        // .arg(Arg::with_name("FILE")
        //     .required(true)
        //     .short("f")
        //     .long("file")
        //     .value_name("FILE")
        //     .help("Sets the input file to use")
        //     .takes_value(true))
        // .arg(Arg::with_name("CONCURRENCY")
        //     .short("c")
        //     .long("concurrency")
        //     .value_name("CONCURRENCY")
        //     .help("Sets the concurrency value - default is 12")
        //     .takes_value(true))
        // .arg(Arg::with_name("RATE_LIMIT")
        //     .short("r")
        //     .long("rate-limit")
        //     .value_name("RATE_LIMIT")
        //     .help("Sets the rate limit value - default is 36")
        //     .takes_value(true))

        let mut command = Command {
            object: "".to_string(),
            action: "".to_string(),
            input_file: "".to_string(),
            prod_spec_assign_file: "".to_string(),
            sku_spec_allowed_values_file: "".to_string(),
            product_file: "".to_string(),
            concurrency: 1,
            rate_limit: NonZeroU32::new(1).unwrap()
        };

        match matches.subcommand() {
            ("category", Some(m)) => {
                command.object = "category".to_string();
                command.input_file = m.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/categories.csv").to_string();
                debug!("input_file: {}", command.input_file);
            },
            ("brand", Some(m)) => {
                command.object = "brand".to_string();
                command.input_file = m.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/brands.csv").to_string();
                debug!("input_file: {}", command.input_file);
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24");
            },
            ("specificationgroup", Some(m)) => {
                command.object = "specificationgroup".to_string();
                command.input_file = m.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/specificationgrouops.csv").to_string();
                debug!("input_file: {}", command.input_file);
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24");
            },
            ("specification", Some(m)) => {
                command.object = "specification".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/specifications.csv").to_string();
                debug!("input_file: {}", command.input_file);
                command.prod_spec_assign_file = m.value_of("PRODUCT_SPEC_ASSIGNMENTS_FILE").unwrap_or("").to_string();
                command.sku_spec_allowed_values_file = m.value_of("SKU_SPEC_ALLOWED_VALUES_FILE").unwrap_or("").to_string();
                command.product_file = m.value_of("PRODUCT_FILE").unwrap_or("").to_string();
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24");
            },
            ("product", Some(m)) => {
                command.object = "product".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/products.csv").to_string();
                debug!("input_file: {}", command.input_file);
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24. Default is 1 - Recommended");
                command.rate_limit = m.value_of("RATE_LIMIT").unwrap_or("40").parse::<NonZeroU32>().expect("RATE_LIMIT must be a positive integer between 1 and 200. Default is 40 - Recommended");
            },
            _ => error!("no match"),
        }

        // let vtex_object = matches.value_of("OBJECT").expect("-o <OBJECT> must be set (example: Category, Brand, etc.");
        // debug!("vtex_object: {}", vtex_object);
        // let vtex_object1 = match vtex_object {
        //     Some(vtex_object1) => { vtex_object1 }
        //     None => { return Err("-o <OBJECT> must be set (example: category, brand, etc.)") }
        // };
        // let input_file = matches.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/categories.csv");
        // let concurrency = matches.value_of("CONCURRENCY").unwrap_or("12").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24");
        // let rate_limit = matches.value_of("RATE_LIMIT").unwrap_or("36").parse::<u32>().expect("RATE_LIMIT must be a positive integer between 1 and 999");
        // info!("rate_limit: {}", rate_limit);
        // let rate_limit = NonZeroU32::new(rate_limit).unwrap();

        command
    }

    // fn validate_vtex_object(v: String) -> Result<(), String> {
    //     let valid_objects = ["category", "brand", "group", "specification", "fieldvalue", "product", "sku", "productspecification", "skuspecassignment", "skufile", "price", "inventory"];
    //     if valid_objects.contains(&v.as_str()) { return Ok(()); }
    //     Err(String::from("Must set a valid VTEX object: category, brand, group, specification, fieldvalue, product, sku, productspecification, skuspecassignment, skufile, price, inventory"))
    // }
}

pub async fn run() -> Result<(), Box<dyn Error>> {

    let cmd = Command::get_command();
    debug!("command: {:?}", cmd);
    dotenv::dotenv().expect("Failed to read .env file");
    let account_name = env::var("ACCOUNT_NAME").expect("Failed to parse ACCOUNT_NAME");
    let environment = env::var("ENVIRONMENT").expect("Failed to parse ENVIRONMENT");
    let vtex_api_key = env::var("VTEX_API_APPKEY").expect("Failed to parse VTEX_API_APPKEY in .env");
    let vtex_api_apptoken = env::var("VTEX_API_APPTOKEN").expect("Failed to parse VTEX_API_APPTOKEN in .env");
    // let category_url = env::var("CATEGORY_URL").expect("Failed to parse CATEGORY_URL in .env");
    // let brand_url = env::var("BRAND_URL").expect("Failed to parse BRAND_URL in .env");
    // let group_url = env::var("GROUP_URL").expect("Failed to parse GROUP_URL in .env");
    // let specification_url = env::var("SPECIFICATION_URL").expect("Failed to parse SPECIFICATION_URL in .env");
    let fieldvalues_url = env::var("FIELDVALUES_URL").expect("Failed to parse FIELDVALUES_URL in .env");
    // let products_url = env::var("PRODUCTS_URL").expect("Failed to parse PRODUCTS_URL in .env");
    let sku_url = env::var("SKU_URL").expect("Failed to parse SKU_URL in .env");
    let prod_spec_url = env::var("PRODUCT_SPECIFICATION_URL").expect("Failed to parse PRODUCT_SPECIFICATION_URL in .env");
    let sku_spec_url = env::var("SKU_SPECIFICATION_URL").expect("Failed to parse SKU_SPECIFICATION_URL in .env");
    let sku_file_url = env::var("SKU_FILE_URL").expect("Failed to parse SKU_FILE_URL in .env");
    // let price_url = env::var("PRICE_URL").expect("Failed to parse PRICE_URL in .env");
    // let inventory_url = env::var("INVENTORY_URL").expect("Failed to parse INVENTORY_URL in .env");

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
        debug!("before call to load_categories(): {:?}", env::current_dir()?);
        categories::load_categories(cmd.input_file.to_string(), &client, account_name, environment).await?;
    } else if cmd.object.eq("brand") {
        // Load Brands
        debug!("before call to load_brands(): {:?}", env::current_dir()?);
        brands::load_brands(cmd.input_file.to_string(), &client, account_name, environment, cmd.concurrency).await?;
    } else if cmd.object.eq("specificationgroup") {
        // Load specification groups
        specificationgroups::load_specification_groups(cmd.input_file.to_string(), &client, account_name, environment, cmd.concurrency).await?;
    } else if cmd.object.eq("specification") {
        // Load specifications
        if cmd.action.eq("import") {
            specifications::load_specifications(cmd.input_file.to_string(), &client, account_name, environment, cmd.concurrency).await?;
            info!("finished loading specifications");
        } else if cmd.action.eq("genproductspecsfile") {
            specifications::gen_product_specifications_file(cmd.input_file.to_string(), &client, account_name, environment, cmd.prod_spec_assign_file, cmd.product_file).await?;
            info!("finished loading specifications");
        } else if cmd.action.eq("genskuspecsfile") {
            specifications::gen_sku_specifications_file(cmd.input_file.to_string(), &client, account_name, environment, cmd.sku_spec_allowed_values_file, cmd.product_file).await?;
            info!("finished loading specifications");
        }
    } else if cmd.object.eq("fieldvalue") {
        // Load field values
        fieldvalues::load_field_values(cmd.input_file.to_string(), &client, fieldvalues_url).await?;
        info!("finished loading fieldvalues");
    } else if cmd.object.eq("product") {
        // Load products
        if cmd.action.eq("import") {
            products::load_products(cmd.input_file.to_string(), &client, account_name, environment, cmd.concurrency, cmd.rate_limit).await?;
            info!("finished loading products");
        }
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
        prices::load_prices(cmd.input_file.to_string(), &client, account_name, environment, cmd.concurrency, cmd.rate_limit).await?;
        info!("finished loading prices");
    } else if cmd.object.eq("inventory") {
        // Load sku files
        inventory::load_inventory(cmd.input_file.to_string(), &client, account_name, environment, cmd.concurrency).await?;
        info!("finished loading inventory");
    } else {
        info!("Did not enter a valid object - category or brand");
    }

    Ok(())
}
