
use reqwest::blocking::Client;
use vtex::model::{ Specification };
// use serde::{Deserialize, Serialize}; // 1.0.131
use std::{error::Error, collections::HashSet};
use std::fs::File;
use std::env;
use crate::csvrecords::{ ProdAttrAllowedValuesRecord };
use crate::utils;

pub fn build_product_specs_file(client: &Client, groups_url: String, category_tree_url: String) -> Result<(), Box<dyn Error>> {

    // Read in the Specificaiton Groups and store in a HashMap for lookup
    let groups = utils::get_vtex_field_groups(client, groups_url);
    let group_lookup = utils::parse_spec_groups(groups);
    println!("group_lookup: {:?}", group_lookup.len());
    let prod_spec_id = group_lookup.get("Product Specifications").unwrap();

    // Read in the category tree and store in a HashMap for lookup
    let categories = utils::get_vtex_category_tree(client, category_tree_url);
    let category_lookup = utils::parse_category_tree(categories);
    println!("category_lookup: {:?}", category_lookup.len());

    // Get a lookup HashMap for the parent category of a product
    let product_parent_category_lookup = utils::create_product_parent_category_lookup();
    println!("product_parent_category_lookkup: {:?}", product_parent_category_lookup.len());
    // Get a lookup for the cateogory name of a category by GroupIdentifier
    let category_identifier_name_lookup = utils::create_category_name_lookup();
    println!("category_identifier_name_lookup: {:?}", category_identifier_name_lookup.len());

    // Setup the input and output files
    println!("current_directory: {:?}", env::current_dir());
    let in_file = File::open("transform/data/in/ProductAttributeAllowedValues-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = "transform/data/out/ProductSpecsForFieldValues.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    // Create a HashSet to store unique values
    let mut unique_spec_cat: HashSet<String> = HashSet::new();

    // Process the input file
    for line in reader.deserialize() {
        let record: ProdAttrAllowedValuesRecord = line.unwrap();
        // look up the part number
        let parent_cat_identifier = product_parent_category_lookup.get(&record.part_number).unwrap();
        // look up the category name
        let parent_cat_name = category_identifier_name_lookup.get(&parent_cat_identifier.to_string()).unwrap();
        // Look up the VTEX Category Id
        // println!("PartNumber: {}  parent_cat_name: {}", &record.part_number, &parent_cat_name.to_string());
        let vtex_cat_id = category_lookup.get(&parent_cat_name.to_string()).unwrap();
        // println!("vtex_cat_id: {}", vtex_cat_id);

        // Only write a record if the Specification for the given category has not been written
        let unique_spec_cat_id: String = record.name.clone() + String::as_str(&vtex_cat_id.to_string());
        if !unique_spec_cat.contains(&unique_spec_cat_id) {
            println!("unique_spec_cat_id: {}", unique_spec_cat_id);
            let spec = Specification {
                id: None,
                field_type_id: 6, // 6 = Radio
                category_id: Some(vtex_cat_id.clone()),
                field_group_id: prod_spec_id.clone(),
                name: record.name.clone(),
                description: Some(record.name.clone()),
                position: Some(1),
                is_filter: Some(true),
                is_required: Some(false),
                is_on_product_details: Some(true),
                is_stock_keeping_unit: Some(true),
                is_wizard: Some(false),
                is_active: Some(true),
                is_top_menu_link_active: Some(false),
                default_value: None,
            };

            // Write the record
            writer.serialize(spec)?;
            // Add the unique_spec_cat_id to the set
            unique_spec_cat.insert(unique_spec_cat_id);
        }

    }
    // Flush the records
    writer.flush()?;

    Ok(())

}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::error::Error;
    use std::io::BufReader;
    use std::collections::HashMap;
    use vtex::model::Group;
    use vtex::model::CategoryTree;

    use crate::csvrecords::{CatRecord, ProdHeaderRecord};
    // use crate::productspecs;

    // #[test]
    // fn build_product_spec() {
    //     let x = productspecs::build_product_specs_file();
    // }

    #[test]
    fn parse_spec_groups() -> Result<(), Box<dyn Error>> {
        let file = File::open("test_data/groups.json")?;
        let reader = BufReader::new(file);
        let result: Vec<Group> = serde_json::from_reader(reader)?;
        // println!("test result Vec<Group>: {:?}", result);
        let mut group_ids: HashMap<String, i32> = HashMap::new();
        for group in result {
            group_ids.insert(group.name.clone(), group.id.unwrap().clone());
        }
        println!("HasMap Group ids: {:?}", group_ids);
        Ok(())

    }

    #[test]
    fn parse_category_tree() {
        let file = File::open("test_data/categories.json").unwrap();
        let reader = BufReader::new(file);
        let result: Vec<CategoryTree> = serde_json::from_reader(reader).unwrap();
        // println!("test result Vec<CategoryTree>: {:?}", result);
        let mut category_ids: HashMap<String, i32> = HashMap::new();
        for category in result {
            category_ids.insert(category.name.clone(), category.id.clone());
            if category.has_children {
                for category2 in category.children.expect("missing category") {
                    category_ids.insert(category2.name.clone(), category2.id.clone());
                    if category2.has_children {
                       for category3 in category2.children.expect("missing category") {
                           category_ids.insert(category3.name.clone(), category3.id.clone());
                       }
                    }
                }

            }
        }
        println!("HashMap Category Ids: {:?}", category_ids);
    }

    #[test]
    fn create_category_name_lookup() {
        // println!("before file open. pwd: {:?}", env::current_dir());
        let file = File::open("data/in/DeptCatalog-sorted-subset.csv").unwrap();
        let mut reader = csv::Reader::from_reader(file);
        let mut cat_identifier_lookup: HashMap<String, String> = HashMap::new();

        let mut x = 0;
        for line in reader.deserialize() {
            let record: CatRecord = line.unwrap();
            cat_identifier_lookup.insert(record.group_identifier.clone(), record.name.clone());
            x = x + 1;
        }
        assert_eq!(x, cat_identifier_lookup.len());
        // println!("HashMap Category Identifiers: {:?}", cat_identifier_lookup);
    }

    #[test]
    fn create_product_parent_category_lookup() {
        let file = File::open("data/in/ProductHeaderProduct-sorted-subset.csv").unwrap();
        let mut reader = csv::Reader::from_reader(file);
        let mut product_parent_category: HashMap<String, String> = HashMap::new();

        let mut x = 0;
        for line in reader.deserialize() {
            let record: ProdHeaderRecord = line.unwrap();
            product_parent_category.insert(record.part_number.clone(), record.parent_group_identifier.clone());
            x = x + 1;
        }
        // println!("HashMap Category Identifiers: {:?}", product_parent_category);
        assert_eq!(x, product_parent_category.len());

    }
}
