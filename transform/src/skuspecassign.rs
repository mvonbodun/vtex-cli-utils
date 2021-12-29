use std::error::Error;
use std::fs::File;
use reqwest::blocking::Client;
use vtex::model::SkuSpecAssignment;

use crate::csvrecords::SkuDefineAttrValueRecord;
use crate::utils;

pub fn build_sku_spec_assign_file(client: &Client, category_tree_url: String, spec_fields_for_category_url: String, field_values_for_field_url: String) -> Result<(), Box<dyn Error>> {

    // Build a Sku_id lookup fn
    let sku_id_lookup = utils::create_sku_id_lookup();
    println!("sku_id_lookup: {}", sku_id_lookup.len());
    // Build a sku parent category lookup
    let sku_parent_category_lookup = utils::create_sku_parent_category_lookup();
    println!("sku_parent_category_lookup: {}", sku_parent_category_lookup.len());
    // Build a category name lookup
    let category_name_lookup = utils::create_category_name_lookup();
    println!("category_name_lookup: {}", category_name_lookup.len());
    // Build category id lookup
    let category_id_lookup = utils::create_category_id_lookup(client, category_tree_url);
    println!("category_id_lookup: {}", category_id_lookup.len());
    // Build a field id lookup fn get the fields for a category
    let field_id_lookup = utils::create_field_id_lookup(&category_id_lookup, client, spec_fields_for_category_url);
    println!("field_id_lookup: {:?}", field_id_lookup.len());
    // Build a field value id lookup table
    let field_value_id_lookup = utils::create_field_value_id_lookup(&field_id_lookup, client, field_values_for_field_url);
    println!("field_value_id_lookup: {:?}", field_value_id_lookup.len());

    // Setup the input and output files
    let in_file = File::open("transform/data/in/SKUDefiningAttributeValue-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = "transform/data/out/SkuSpecificationAssignment.csv";
    let mut writer = csv::Writer::from_path(out_path)?;
    
    let mut x = 0;
    for line in reader.deserialize() {
        let record: SkuDefineAttrValueRecord = line?;
        
        // Get the category identifier for the partnumber
        let parent_category_identifier = sku_parent_category_lookup.get(&record.part_number).unwrap();
        // Get category name
        let parent_cat_name = category_name_lookup.get(parent_category_identifier).unwrap();
        // Get the VTEX Category Id
        let vtex_cat_id = category_id_lookup.get(parent_cat_name).unwrap();
        // Build the key to use with field_id_lookup
        let key = vtex_cat_id.to_string().to_owned() + "|" + record.name.as_str();
        // Get the field_id
        let field_id = field_id_lookup.get(&key).expect("failed to find field_id in field_id_lookup");
        // Build the key to use with the field_value_id_lookup
        let field_value_key = field_id.to_string().as_str().to_owned() + "|" + record.value.as_str().trim();
        let field_value_id = field_value_id_lookup.get(&field_value_key).expect("failed to find field_value_id in field_value_id_lookup");
        println!("record.part_number {}", &record.part_number);
        if sku_id_lookup.contains_key(&record.part_number) {
            let sku_spec_assign = SkuSpecAssignment {
                id: Some(0), // Hardcode to 0, API does not work with None (null)
                sku_id: *sku_id_lookup.get(&record.part_number).unwrap(),
                field_id: field_id.clone(),
                field_value_id: Some(field_value_id.clone()),
                text: None,
            };
            writer.serialize(sku_spec_assign)?;
            x = x + 1;
        }
    }
    // Flush the records
    writer.flush()?;
    println!("records written: {}", x);

    Ok(())
}