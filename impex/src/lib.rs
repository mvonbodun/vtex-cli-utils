use clap::{arg_enum, crate_version, App, Arg, SubCommand};
use log::*;
use reqwest::header;
use std::error::Error;
use std::io::Write;
use std::num::NonZeroU32;
use std::sync::Once;
use std::{env, time::Duration};

mod brands;
mod categories;
mod csvrecords;
mod inventory;
mod prices;
mod products;
mod productspecassociation;
mod similarcategories;
mod skuean;
mod skufiles;
mod skus;
mod skuspecassociation;
mod specificationgroups;
mod specifications;
mod specificationvalues;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        let start = std::time::Instant::now();
        env_logger::Builder::from_default_env()
            .format(move |buf, rec| {
                let t = start.elapsed().as_secs_f32();
                writeln!(buf, "{:.03} [{}] - {}", t, rec.level(), rec.args())
            })
            .init();
    })
}

#[derive(Debug)]
struct Command {
    object: String,
    action: String,
    input_file: String,
    prod_spec_assign_file: String,
    sku_spec_allowed_values_file: String,
    sku_spec_assign_file: String,
    product_file: String,
    sku_file: String,
    concurrency: usize,
    rate_limit: NonZeroU32,
    skip_cat_lookup: usize,
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum CategoryActions {
        import,
        update
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum BrandActions {
        import,
        genbrandfile
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
    enum SpecificationValueActions {
        import,
        genspecvaluesfile,
        genspecvaluesfilealternate
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum ProductActions {
        import,
        update
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum SkuActions {
        import
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum ProductSpecAssocActions {
        import,
        genproductspecassocfile,
        genproductspecassocfilerootcategory
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum SkuSpecAssocActions {
        import,
        genskuspecassocfile,
        genskuspecassocfilealternate
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum SkuFileActions {
        import,
        genskufile
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum SkuEanActions {
        import,
        genskueanfile
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum SimilarCategoryActions {
        import
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum PriceActions {
        import
    }
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum InventoryActions {
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
                .help("The action to perform on the VTEX Object - import, update")
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
        .subcommand(SubCommand::with_name("specificationvalue")
            .about("actions on the specification value into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&SpecificationValueActions::variants())
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
            .arg(Arg::with_name("RATELIMIT")
                .short("r")
                .long("rate_limit")
                .value_name("RATELIMIT")
                .help("Sets the rate limit value (how many calls per second) - default is 40")
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
                .help("Sets the rate limit value (how many calls per second) - default is 40")
                .takes_value(true))
            .arg(Arg::with_name("SKIPCATLOOKUP")
                .short("s")
                .long("skip_cat_lookup")
                .value_name("SKIPCATLOOKUP")
                .help("If you pass in the category_id it will skip building the category id lookups")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("sku")
            .about("actions on the sku into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&SkuActions::variants())
                .short("a")
                .long("action")
                .value_name("ACTION")
                .help("The action to perform on the VTEX Object: - import, export")
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
                .help("Sets the rate limit value (how many calls per second) - default is 40")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("productspecassociation")
            .about("actions on product specification associations into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&ProductSpecAssocActions::variants())
                .short("a")
                .long("action")
                .value_name("ACTION")
                .help("The action to perform on the VTEX Object: ")
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
            .arg(Arg::with_name("RATELIMIT")
                .short("r")
                .long("rate_limit")
                .value_name("RATELIMIT")
                .help("Sets the rate limit value (how many calls per second) - default is 40")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("skuspecassociation")
            .about("actions on sku specification associations into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&SkuSpecAssocActions::variants())
                .short("a")
                .long("action")
                .value_name("ACTION")
                .help("The action to perform on the VTEX Object: ")
                .takes_value(true))
            .arg(Arg::with_name("FILE")
                .required(true)
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets the input or output file to read or write to.")
                .takes_value(true))
            .arg(Arg::with_name("SKU_SPEC_ASSIGNMENTS_FILE")
                .required(false)
                .long("sku_spec_assigns_file")
                .value_name("SKU_SPEC_ASSIGNMENTS_FILE")
                .help("Sets the Sku Specification Assignments file")
                .takes_value(true))
            .arg(Arg::with_name("PRODUCT_FILE")
                .required(false)
                .long("product_file")
                .value_name("PRODUCT_FILE")
                .help("Sets the Product file")
                .takes_value(true))
            .arg(Arg::with_name("SKU_FILE")
                .required(false)
                .long("sku_file")
                .value_name("SKU_FILE")
                .help("Sets the Sku file")
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
                .help("Sets the rate limit value (how many calls per second) - default is 40")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("skufile")
            .about("actions on skufile (images) into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&SkuFileActions::variants())
                .short("a")
                .long("action")
                .value_name("ACTION")
                .help("The action to perform on the VTEX Object: ")
                .takes_value(true))
            .arg(Arg::with_name("FILE")
                .required(true)
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets the input or output file to read or write to.")
                .takes_value(true))
            .arg(Arg::with_name("SKU_FILE")
                .required(false)
                .long("sku_file")
                .value_name("SKU_FILE")
                .help("Sets the Sku file")
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
                .help("Sets the rate limit value (how many calls per second) - default is 40")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("skuean")
            .about("actions on skuean into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&SkuEanActions::variants())
                .short("a")
                .long("action")
                .value_name("ACTION")
                .help("The action to perform on the VTEX Object: ")
                .takes_value(true))
            .arg(Arg::with_name("FILE")
                .required(true)
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Sets the input or output file to read or write to.")
                .takes_value(true))
            .arg(Arg::with_name("SKU_FILE")
                .required(false)
                .long("sku_file")
                .value_name("SKU_FILE")
                .help("Sets the Sku file")
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
                .help("Sets the rate limit value (how many calls per second) - default is 40")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("similarcategory")
            .about("actions on similarcategory into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&SimilarCategoryActions::variants())
                .short("a")
                .long("action")
                .value_name("ACTION")
                .help("The action to perform on the VTEX Object: ")
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
        .subcommand(SubCommand::with_name("price")
            .about("actions on the price into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&PriceActions::variants())
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
                .help("Sets the rate limit value (how many calls per second) - default is 30")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("inventory")
            .about("actions on the inventory into VTEX")
            .version(crate_version!())
            .arg(Arg::with_name("ACTION")
                .required(true)
                .possible_values(&InventoryActions::variants())
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
                .help("Sets the rate limit value (how many calls per second) - default is 40")
                .takes_value(true))
        )
        .get_matches();

        let mut command = Command {
            object: "".to_string(),
            action: "".to_string(),
            input_file: "".to_string(),
            prod_spec_assign_file: "".to_string(),
            sku_spec_allowed_values_file: "".to_string(),
            sku_spec_assign_file: "".to_string(),
            product_file: "".to_string(),
            sku_file: "".to_string(),
            concurrency: 1,
            rate_limit: NonZeroU32::new(1).unwrap(),
            skip_cat_lookup: 0,
        };

        match matches.subcommand() {
            ("category", Some(m)) => {
                command.object = "category".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m
                    .value_of("FILE")
                    .expect("-f <FILE> must be set to the input file (example: data/categories.csv")
                    .to_string();
                debug!("input_file: {}", command.input_file);
            }
            ("brand", Some(m)) => {
                command.object = "brand".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m
                    .value_of("FILE")
                    .expect("-f <FILE> must be set to the input file (example: data/brands.csv")
                    .to_string();
                debug!("input_file: {}", command.input_file);
                command.product_file = m.value_of("PRODUCT_FILE").unwrap_or("").to_string();
                command.concurrency = m
                    .value_of("CONCURRENCY")
                    .unwrap_or("1")
                    .parse::<usize>()
                    .expect("CONCURRENCY must be a positive integer between 1 and 24");
            }
            ("specificationgroup", Some(m)) => {
                command.object = "specificationgroup".to_string();
                command.input_file = m.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/specificationgrouops.csv").to_string();
                debug!("input_file: {}", command.input_file);
                command.concurrency = m
                    .value_of("CONCURRENCY")
                    .unwrap_or("1")
                    .parse::<usize>()
                    .expect("CONCURRENCY must be a positive integer between 1 and 24");
            }
            ("specification", Some(m)) => {
                command.object = "specification".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m
                    .value_of("FILE")
                    .expect(
                        "-f <FILE> must be set to the input file (example: data/specifications.csv",
                    )
                    .to_string();
                debug!("input_file: {}", command.input_file);
                command.prod_spec_assign_file = m
                    .value_of("PRODUCT_SPEC_ASSIGNMENTS_FILE")
                    .unwrap_or("")
                    .to_string();
                command.sku_spec_allowed_values_file = m
                    .value_of("SKU_SPEC_ALLOWED_VALUES_FILE")
                    .unwrap_or("")
                    .to_string();
                command.product_file = m.value_of("PRODUCT_FILE").unwrap_or("").to_string();
                command.concurrency = m
                    .value_of("CONCURRENCY")
                    .unwrap_or("1")
                    .parse::<usize>()
                    .expect("CONCURRENCY must be a positive integer between 1 and 24");
            }
            ("specificationvalue", Some(m)) => {
                command.object = "specificationvalue".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/specificationvalues.csv").to_string();
                debug!("input_file: {}", command.input_file);
                command.sku_spec_allowed_values_file = m
                    .value_of("SKU_SPEC_ALLOWED_VALUES_FILE")
                    .unwrap_or("")
                    .to_string();
                command.product_file = m.value_of("PRODUCT_FILE").unwrap_or("").to_string();
                command.concurrency = m
                    .value_of("CONCURRENCY")
                    .unwrap_or("1")
                    .parse::<usize>()
                    .expect("CONCURRENCY must be a positive integer between 1 and 24");
                command.rate_limit = m.value_of("RATE_LIMIT").unwrap_or("40").parse::<NonZeroU32>().expect("RATE_LIMIT must be a positive integer between 1 and 200. Default is 40 - Recommended");
            }
            ("product", Some(m)) => {
                command.object = "product".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m
                    .value_of("FILE")
                    .expect("-f <FILE> must be set to the input file (example: data/products.csv")
                    .to_string();
                debug!("input_file: {}", command.input_file);
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24. Default is 1 - Recommended");
                command.rate_limit = m.value_of("RATE_LIMIT").unwrap_or("40").parse::<NonZeroU32>().expect("RATE_LIMIT must be a positive integer between 1 and 200. Default is 40 - Recommended");
                command.skip_cat_lookup = m.value_of("SKIPCATLOOKUP").unwrap_or("0").parse::<usize>().expect("SKIPCATLOOKUP must be a 0 or 1. Default is 0 - perform category lookup, 1 will skip the category lookup");
            }
            ("sku", Some(m)) => {
                command.object = "sku".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m
                    .value_of("FILE")
                    .expect("-f <FILE> must be set to the input file (example: data/skus.csv")
                    .to_string();
                debug!("input_file: {}", command.input_file);
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24. Default is 1 - Recommended");
                command.rate_limit = m.value_of("RATE_LIMIT").unwrap_or("40").parse::<NonZeroU32>().expect("RATE_LIMIT must be a positive integer between 1 and 200. Default is 40 - Recommended");
            }
            ("productspecassociation", Some(m)) => {
                command.object = "productspecassociation".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/ProductSpecificationAssociation.csv").to_string();
                debug!("input_file: {}", command.input_file);
                command.prod_spec_assign_file = m
                    .value_of("PRODUCT_SPEC_ASSIGNMENTS_FILE")
                    .unwrap_or("")
                    .to_string();
                command.product_file = m.value_of("PRODUCT_FILE").unwrap_or("").to_string();
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24. Default is 1 - Recommended");
                command.rate_limit = m.value_of("RATE_LIMIT").unwrap_or("40").parse::<NonZeroU32>().expect("RATE_LIMIT must be a positive integer between 1 and 200. Default is 40 - Recommended");
            }
            ("skuspecassociation", Some(m)) => {
                command.object = "skuspecassociation".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m.value_of("FILE").expect("-f <FILE> must be set to the input file (example: data/SkuSpecificationAssociation.csv").to_string();
                debug!("input_file: {}", command.input_file);
                command.sku_spec_assign_file = m
                    .value_of("SKU_SPEC_ASSIGNMENTS_FILE")
                    .unwrap_or("")
                    .to_string();
                command.product_file = m.value_of("PRODUCT_FILE").unwrap_or("").to_string();
                command.sku_file = m.value_of("SKU_FILE").unwrap_or("").to_string();
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24. Default is 1 - Recommended");
                command.rate_limit = m.value_of("RATE_LIMIT").unwrap_or("40").parse::<NonZeroU32>().expect("RATE_LIMIT must be a positive integer between 1 and 200. Default is 40 - Recommended");
            }
            ("skufile", Some(m)) => {
                command.object = "skufile".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m
                    .value_of("FILE")
                    .expect("-f <FILE> must be set to the input file (example: data/SkuFile.csv")
                    .to_string();
                debug!("input_file: {}", command.input_file);
                command.sku_file = m.value_of("SKU_FILE").unwrap_or("").to_string();
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24. Default is 1 - Recommended");
                command.rate_limit = m.value_of("RATE_LIMIT").unwrap_or("40").parse::<NonZeroU32>().expect("RATE_LIMIT must be a positive integer between 1 and 200. Default is 40 - Recommended");
            }
            ("skuean", Some(m)) => {
                command.object = "skuean".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m
                    .value_of("FILE")
                    .expect("-f <FILE> must be set to the input file (example: data/SkuEan.csv")
                    .to_string();
                debug!("input_file: {}", command.input_file);
                command.sku_file = m.value_of("SKU_FILE").unwrap_or("").to_string();
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24. Default is 1 - Recommended");
                command.rate_limit = m.value_of("RATE_LIMIT").unwrap_or("40").parse::<NonZeroU32>().expect("RATE_LIMIT must be a positive integer between 1 and 200. Default is 40 - Recommended");
            }
            ("similarcategory", Some(m)) => {
                command.object = "similarcategory".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m
                    .value_of("FILE")
                    .expect("-f <FILE> must be set to the input file (example: data/SkuEan.csv")
                    .to_string();
                debug!("input_file: {}", command.input_file);
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24. Default is 1 - Recommended");
            }
            ("price", Some(m)) => {
                command.object = "price".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m
                    .value_of("FILE")
                    .expect("-f <FILE> must be set to the input file (example: data/SkuFile.csv")
                    .to_string();
                debug!("input_file: {}", command.input_file);
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("2").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24. Default is 2 - Recommended");
                command.rate_limit = m.value_of("RATE_LIMIT").unwrap_or("30").parse::<NonZeroU32>().expect("RATE_LIMIT must be a positive integer between 1 and 200. Default is 30 - Recommended");
            }
            ("inventory", Some(m)) => {
                command.object = "inventory".to_string();
                command.action = m.value_of("ACTION").unwrap().to_string();
                command.input_file = m
                    .value_of("FILE")
                    .expect("-f <FILE> must be set to the input file (example: data/SkuFile.csv")
                    .to_string();
                debug!("input_file: {}", command.input_file);
                command.concurrency = m.value_of("CONCURRENCY").unwrap_or("1").parse::<usize>().expect("CONCURRENCY must be a positive integer between 1 and 24. Default is 1 - Recommended");
                command.rate_limit = m.value_of("RATE_LIMIT").unwrap_or("40").parse::<NonZeroU32>().expect("RATE_LIMIT must be a positive integer between 1 and 200. Default is 40 - Recommended");
            }
            _ => error!("no match"),
        }

        command
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let cmd = Command::get_command();
    debug!("command: {:?}", cmd);
    dotenv::dotenv().expect("Failed to read .env file");
    let account_name = env::var("ACCOUNT_NAME").expect("Failed to parse ACCOUNT_NAME");
    let environment = env::var("ENVIRONMENT").expect("Failed to parse ENVIRONMENT");
    let vtex_api_key =
        env::var("VTEX_API_APPKEY").expect("Failed to parse VTEX_API_APPKEY in .env");
    let vtex_api_apptoken =
        env::var("VTEX_API_APPTOKEN").expect("Failed to parse VTEX_API_APPTOKEN in .env");

    // Setup the HTTP client
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "X-VTEX-API-AppKey",
        header::HeaderValue::from_str(&vtex_api_key)?,
    );
    headers.insert(
        "X-VTEX-API-AppToken",
        header::HeaderValue::from_str(&vtex_api_apptoken)?,
    );
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .default_headers(headers)
        .build()?;

    if cmd.object.eq("category") {
        if cmd.action.eq("import") {
            // Load Categories
            debug!(
                "before call to load_categories(): {:?}",
                env::current_dir()?
            );
            categories::load_categories(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
            )
            .await?;
        } else if cmd.action.eq("update") {
            debug!(
                "before call to update_categories(): {:?}",
                env::current_dir()?
            );
            categories::update_categories(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
            )
            .await?;
        }
    } else if cmd.object.eq("brand") {
        if cmd.action.eq("import") {
            // Load Brands
            debug!("before call to load_brands(): {:?}", env::current_dir()?);
            brands::load_brands(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
            )
            .await?;
        } else if cmd.action.eq("genbrandfile") {
            brands::gen_brand_file(cmd.input_file.to_string(), cmd.product_file)?;
        }
    } else if cmd.object.eq("specificationgroup") {
        // Load specification groups
        specificationgroups::load_specification_groups(
            cmd.input_file.to_string(),
            &client,
            account_name,
            environment,
            cmd.concurrency,
        )
        .await?;
    } else if cmd.object.eq("specification") {
        // Load specifications
        if cmd.action.eq("import") {
            specifications::load_specifications(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
            )
            .await?;
        } else if cmd.action.eq("genproductspecsfile") {
            specifications::gen_product_specifications_file(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.prod_spec_assign_file,
                cmd.product_file,
            )
            .await?;
        } else if cmd.action.eq("genskuspecsfile") {
            specifications::gen_sku_specifications_file(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.sku_spec_allowed_values_file,
                cmd.product_file,
            )
            .await?;
        }
    } else if cmd.object.eq("specificationvalue") {
        if cmd.action.eq("import") {
            // Load field values
            specificationvalues::load_specification_values(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
                cmd.rate_limit,
            )
            .await?;
        } else if cmd.action.eq("genspecvaluesfile") {
            specificationvalues::gen_specification_values_file(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.sku_spec_allowed_values_file,
                cmd.product_file,
            )
            .await?;
        } else if cmd.action.eq("genspecvaluesfilealternate") {
            specificationvalues::gen_specification_values_file_alternate(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.sku_spec_allowed_values_file,
            )
            .await?
        }
    } else if cmd.object.eq("product") {
        // Load products
        if cmd.action.eq("import") {
            products::load_products(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
                cmd.rate_limit,
                cmd.skip_cat_lookup,
            )
            .await?;
        } else if cmd.action.eq("update") {
            products::update_products(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
                cmd.rate_limit,
                cmd.skip_cat_lookup,
            )
            .await?;
        }
    } else if cmd.object.eq("sku") {
        // Load skus
        if cmd.action.eq("import") {
            skus::load_skus(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
                cmd.rate_limit,
            )
            .await?;
        }
    } else if cmd.object.eq("productspecassociation") {
        // Load product specs
        if cmd.action.eq("import") {
            productspecassociation::load_product_spec_associations(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
                cmd.rate_limit,
            )
            .await?;
        } else if cmd.action.eq("genproductspecassocfile") {
            productspecassociation::gen_product_spec_association_file(
                cmd.input_file,
                &client,
                account_name,
                environment,
                cmd.prod_spec_assign_file,
                cmd.product_file,
            )
            .await?;
        } else if cmd.action.eq("genproductspecassocfilerootcategory") {
            productspecassociation::gen_product_spec_association_file_root_category(
                cmd.input_file,
                &client,
                account_name,
                environment,
                cmd.prod_spec_assign_file,
            )
            .await?;
        }
    } else if cmd.object.eq("skuspecassociation") {
        // Load sku spec assignments
        if cmd.action.eq("import") {
            skuspecassociation::load_sku_spec_associations(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
                cmd.rate_limit,
            )
            .await?;
        } else if cmd.action.eq("genskuspecassocfile") {
            skuspecassociation::gen_sku_spec_association_file(
                cmd.input_file,
                &client,
                account_name,
                environment,
                cmd.sku_spec_assign_file,
                cmd.product_file,
                cmd.sku_file,
            )
            .await?;
        } else if cmd.action.eq("genskuspecassocfilealternate") {
            skuspecassociation::gen_sku_spec_assign_file_alternate(
                cmd.input_file,
                &client,
                account_name,
                environment,
                cmd.sku_spec_assign_file,
                cmd.product_file,
                cmd.sku_file,
            )
            .await?
        }
    } else if cmd.object.eq("skufile") {
        // Load sku files
        if cmd.action.eq("import") {
            skufiles::load_sku_files(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
                cmd.rate_limit,
            )
            .await?;
        } else if cmd.action.eq("genskufile") {
            skufiles::gen_sku_file(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.sku_file,
            )
            .await?;
        }
    } else if cmd.object.eq("skuean") {
        // Load sku files
        if cmd.action.eq("import") {
            skuean::load_sku_eans(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
                cmd.rate_limit,
            )
            .await?;
        } else if cmd.action.eq("genskueanfile") {
            skuean::gen_sku_ean_file(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.sku_file,
            )
            .await?;
        }
    } else if cmd.object.eq("similarcategory") {
        // Load similar categories
        if cmd.action.eq("import") {
            similarcategories::load_similar_categories(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
            )
            .await?;
        }
    } else if cmd.object.eq("price") {
        // Load price
        if cmd.action.eq("import") {
            prices::load_prices(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
                cmd.rate_limit,
            )
            .await?;
        }
    } else if cmd.object.eq("inventory") {
        // Load sku files
        if cmd.action.eq("import") {
            inventory::load_inventory(
                cmd.input_file.to_string(),
                &client,
                account_name,
                environment,
                cmd.concurrency,
                cmd.rate_limit,
            )
            .await?;
        }
    } else {
        info!("Did not enter a valid object");
    }

    Ok(())
}
