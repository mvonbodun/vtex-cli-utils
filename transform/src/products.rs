use reqwest::blocking::Client;
use vtex::model::Product;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use crate::{utils, csvrecords};
use crate::csvrecords::ProdHeaderRecord;

pub fn build_product_file(client: &Client, category_tree_url: String, brand_url: String) -> Result<(), Box<dyn Error>> {
  
    // Read in the category tree and store in a HashMap for lookup
    let categories = utils::get_vtex_category_tree(client, category_tree_url);
    let category_lookup = utils::parse_category_tree(categories);
    println!("category_lookup: {:?}", category_lookup.len());

    // Get a lookup for the cateogory name of a category by GroupIdentifier
    let category_identifier_name_lookup = utils::create_category_name_lookup();
    println!("category_identifier_name_lookup: {:?}", category_identifier_name_lookup.len());

    // Build the PartNumber to Brand lookup
    let in_file = File::open("transform/data/in/ProductDescriptiveAttributeValue-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let mut partnumber_brand_lookup: HashMap<String, String> = HashMap::new();
    for line in reader.deserialize() {
        let record: csvrecords::ProdDescAttrRecord = line.unwrap();
        if record.name.eq("Brand") {
            partnumber_brand_lookup.insert(record.part_number.clone(), record.value.clone());
        }
    }

    // Get a lookup for the brand_id by brand name
    let brand_id_lookup = utils::create_brand_lookup(client, brand_url);
    println!("brand_id_lookup: {}", brand_id_lookup.len());

    // Setup the input and output files
    let in_file = File::open("transform/data/in/ProductHeaderProduct-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = "transform/data/out/Products.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    // Process the input file
    let mut x = 0;
    for line in reader.deserialize() {
        let record: ProdHeaderRecord = line.unwrap();

        // look up the category name
        let parent_cat_name = category_identifier_name_lookup.get(&record.parent_group_identifier.to_string()).unwrap();
        // Look up the VTEX Category Id
        // println!("PartNumber: {}  parent_cat_name: {}", &record.part_number, &parent_cat_name.to_string());
        let vtex_cat_id = category_lookup.get(&parent_cat_name.to_string()).unwrap();
        // Look up the brand by partnumber
        let brand = partnumber_brand_lookup.get(&record.part_number.to_string()).unwrap();
        // Look up the brand_id
        let brand_id = brand_id_lookup.get(&brand.to_string()).unwrap();

        let product = Product {
            id: None,
            name: record.name.clone(),
            department_id: None,
            category_id: vtex_cat_id.clone(),
            brand_id: brand_id.clone(),
            link_id: Some(record.name.clone().replace(" ", "-") + "-" + record.part_number.as_str()),
            ref_id: Some(record.part_number.clone()),
            is_visible: Some(true),
            description: Some(record.long_description),
            description_short: Some(record.short_description),
            release_date: Some("2021-10-01T00:00:00".to_string()),
            key_words: Some(record.name.clone().replace(" ", ", ")),
            title: Some(record.name.clone()),
            is_active: Some(true),
            tax_code: None,
            meta_tag_description: Some(record.name.clone()),
            supplier_id: None,
            show_without_stock: Some(true),
            ad_words_remarketing_code: None,
            lomadee_campaign_code: None,
            score: None,
        };

        // Write the record
        writer.serialize(product)?;
        x = x + 1;

    }
    // Flush the records
    writer.flush()?;
    println!("records written: {}", x);

    Ok(())
}