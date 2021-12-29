use csv::StringRecord;
use reqwest::blocking::Client;
use vtex::model::{ CategoryTree, SpecificationValue };
use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::File;
use crate::csvrecords::{ CatRecord, ProdHeaderRecord };


// Get the VTEX Category Tree - to store the Id and Name in a HashMap
fn get_vtex_category_tree(client: &Client, url: String) -> Vec<vtex::model::CategoryTree> {
    let categories: Vec<vtex::model::CategoryTree> = client
            .get(url)
            .send()
            .unwrap()
            .json()
            .unwrap();
    categories
}

// Get the specs for a given category
fn get_spec_fields_for_category(client: &Client, url: String) -> Vec<vtex::model::SpecificationList> {
    let specs: Vec<vtex::model::SpecificationList> = client
            .get(url)
            .send()
            .unwrap()
            .json()
            .unwrap();
    specs
}

// Read in the Category Id
// Parse the Category Tree into a HashMap for Key Lookup
fn parse_category_tree(cat_tree: Vec<CategoryTree>) -> HashMap<String, i32> {
    let mut category_ids: HashMap<String, i32> = HashMap::new();
    for category in cat_tree {
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
    category_ids
}

// Create a lookup HashMap that allows lookup of Category Name from Category GroupIdentifier
fn create_category_name_lookup() -> HashMap<String, String> {
    // println!("before file open. pwd: {:?}", env::current_dir());
    let file = File::open("transform/data/in/DeptCatalog-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    let mut cat_identifier_lookup: HashMap<String, String> = HashMap::new();

    for line in reader.deserialize() {
        let record: CatRecord = line.unwrap();
        cat_identifier_lookup.insert(record.group_identifier.clone(), record.name.clone());
    }
    cat_identifier_lookup
}

// Create a lookup HashMap that allows lookup of the parent category GroupoIdentifier by the PartNumber
fn create_product_parent_category_lookup() -> HashMap<String, String> {
    let file = File::open("transform/data/in/ProductHeaderProduct-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    let mut product_parent_category: HashMap<String, String> = HashMap::new();

    for line in reader.deserialize() {
        let record: ProdHeaderRecord = line.unwrap();
        product_parent_category.insert(record.part_number.clone(), record.parent_group_identifier.clone());
    }
    // println!("HashMap Category Identifiers: {:?}", product_parent_category);
    product_parent_category
}

fn create_field_id_lookup(category_lookup: &HashMap<String, i32>, client: &Client, url: String) -> HashMap<String, i32> {
    // Lookup by [cat_id + field name, field-id]
    let mut field_id_lookup: HashMap<String, i32> = HashMap::new();
    for category in category_lookup {
        // get the fields for the category
        let url = url.to_string() + category.1.to_string().as_str();
        let category_fields = get_spec_fields_for_category(client, url);
        for cat_field in category_fields {
            let key = category.1.to_string().as_str().to_owned() + "|" + cat_field.name.as_str();
            field_id_lookup.insert(key, cat_field.field_id);
        }
    }
    field_id_lookup
}

pub fn build_field_values_file(client: &Client, category_tree_url: String, spec_fields_for_category_url: String) -> Result<(), Box<dyn Error>> {

    // Read in the category tree and store in a HashMap for lookup
    let categories = get_vtex_category_tree(client, category_tree_url);
    let category_lookup = parse_category_tree(categories);
    println!("category_lookup: {:?}", category_lookup.len());

    // Need HashMap to get Field Id
    let field_id_lookup = create_field_id_lookup(&category_lookup, client, spec_fields_for_category_url);
    println!("field_id_lookup: {:?}", field_id_lookup.len());
    // Get a lookup HashMap for the parent category of a product
    let product_parent_category_lookup = create_product_parent_category_lookup();
    println!("product_parent_category_lookkup: {:?}", product_parent_category_lookup.len());
    // Get a lookup for the cateogory name of a category by GroupIdentifier
    let category_identifier_name_lookup = create_category_name_lookup();
    println!("category_identifier_name_lookup: {:?}", category_identifier_name_lookup.len());

    // Setup the input and output files
    println!("current_directory: {:?}", env::current_dir());
    let in_file = File::open("transform/data/in/ProductAttributeAllowedValues-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = "transform/data/out/FieldValues.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut record = StringRecord::new();
    let mut specvalue_set: HashSet<SpecificationValue> = HashSet::new();
    while reader.read_record(&mut record)? {
        let partnumber = record.get(0).unwrap().to_string();
        // look up the part number
        let parent_cat_identifier = product_parent_category_lookup.get(&partnumber).unwrap();
        // look up the category name
        let parent_cat_name = category_identifier_name_lookup.get(&parent_cat_identifier.to_string()).unwrap();
        // Look up the VTEX Category Id
        // println!("PartNumber: {}  parent_cat_name: {}", &record.part_number, &parent_cat_name.to_string());
        let vtex_cat_id = category_lookup.get(&parent_cat_name.to_string()).unwrap();
        // println!("vtex_cat_id: {}", vtex_cat_id);
        let name = record.get(2).unwrap().to_string();
        let key = vtex_cat_id.to_string().to_owned() + "|" + name.as_str();
        let field_id = field_id_lookup.get(&key).expect("failed to find field_id for category in field_id_lookup");
        
        // The AllowedValues fields start in the 4th postion of the file - range begins at 3 in for loop
        for number in 3..record.len() {
            let value = record.get(number).unwrap().trim();
            if value.len() > 0 {
                // println!("name: [{}] value: [{}]", name, value);
                let field_value = SpecificationValue {
                    field_value_id: None,
                    field_id: field_id.clone(),
                    is_active: Some(true),
                    name: value.to_string(),
                    text: None,
                    position: None,
                };
                // Don't insert duplicate records
                if !specvalue_set.contains(&field_value) {
                    writer.serialize(field_value.clone())?;
                    specvalue_set.insert(field_value);
                }
            }
        }
    }
    // Flush the records
    writer.flush()?;

    Ok(())

}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

}