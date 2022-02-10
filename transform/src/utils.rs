use reqwest::{blocking::Client, StatusCode};
use std::{collections::HashMap, env};
use std::fs::File;
use vtex::model::{ BrandList, CategoryTree, SpecificationGroup };
use crate::csvrecords::{ CatRecord, ProdHeaderRecord, ProductLookup, SkuLookup };


// Get the in the Field Groups to store the Id and Name, store in a HashMap
pub fn get_vtex_field_groups(client: &Client, url: String) -> Vec<vtex::model::SpecificationGroup> {
    let groups: Vec<vtex::model::SpecificationGroup> = client.get(url)
            .send()
            .unwrap()
            .json()
            .unwrap();
    groups
}

// Get the VTEX Category Tree - to store the Id and Name in a HashMap
pub fn get_vtex_category_tree(client: &Client, url: String) -> Vec<vtex::model::CategoryTree> {
    let categories: Vec<vtex::model::CategoryTree> = client
            .get(url)
            .send()
            .unwrap()
            .json()
            .unwrap();
    categories
}

// Get the specs for a given category
pub fn get_spec_fields_for_category(client: &Client, url: String) -> Vec<vtex::model::SpecificationList> {
    let specs: Vec<vtex::model::SpecificationList> = client
            .get(url)
            .send()
            .unwrap()
            .json()
            .unwrap();
    specs
}

// Get the field values for a given field
pub fn get_field_values_for_field_id(client: &Client, url: String) -> Vec<vtex::model::FieldValueList> {
    let fieldvalues: Vec<vtex::model::FieldValueList> = client
            .get(url)
            .send()
            .unwrap()
            .json()
            .unwrap();
    fieldvalues
}

// Get the brands
pub fn get_brands(client: &Client, url: String) -> Vec<vtex::model::BrandList> {
    let response = client
        .get(url)
        .send()
        .unwrap();
    match response.status() {
        StatusCode::OK => {
            let result: Vec<BrandList> = response.json().unwrap();
            println!("Vec<Brand> length: {}", result.len());
            return result
        },
        _ => {
            println!("response.status: {}, error: {:#?}", response.status(), response.text().unwrap());
            panic!("failed to get brands");
        },
    }
    // let b_id = brands.iter().map(|b| b.id).collect();
    // brands
}

// Parse the Brands into a HashMap for Key Lookup
pub fn parse_brands(brands: Vec<BrandList>) -> HashMap<String, i32> {
    let mut brand_ids: HashMap<String, i32> = HashMap::new();
    for brand in brands {
        brand_ids.insert(brand.name.clone(), brand.id.clone());
    }
    brand_ids
}

// Create brand lookup
pub fn create_brand_lookup(client: &Client, url: String) -> HashMap<String, i32> {
    parse_brands(get_brands(client, url))
}

// Parse the Specification Groups into a HashMap for Key Lookup
pub fn parse_spec_groups(groups: Vec<SpecificationGroup>) -> HashMap<String, i32> {
    let mut group_ids: HashMap<String, i32> = HashMap::new();
    for group in groups {
        group_ids.insert(group.name.clone(), group.id.unwrap().clone());
    }
    group_ids
}

// Read in the Category Id
// Parse the Category Tree into a HashMap for Key Lookup
pub fn parse_category_tree(cat_tree: Vec<CategoryTree>) -> HashMap<String, i32> {
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

// Create category id lookup HashMap
pub fn create_category_id_lookup(client: &Client, url: String) -> HashMap<String, i32> {
    parse_category_tree(get_vtex_category_tree(client, url))
}

// Create a lookup HashMap that allows lookup of Category Name from Category GroupIdentifier
pub fn create_category_name_lookup() -> HashMap<String, String> {
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
pub fn create_product_parent_category_lookup() -> HashMap<String, String> {
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

// Create a lookup HashMap that allows lookup of the parent category GroupoIdentifier by the PartNumber
pub fn create_sku_parent_category_lookup() -> HashMap<String, String> {
    let file = File::open("transform/data/in/ProductHeaderItem-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    let mut sku_parent_category: HashMap<String, String> = HashMap::new();

    for line in reader.deserialize() {
        let record: ProdHeaderRecord = line.unwrap();
        sku_parent_category.insert(record.part_number.clone(), record.parent_group_identifier.clone());
    }
    // println!("HashMap Category Identifiers: {:?}", sku_parent_category);
    sku_parent_category
}

// Create field_id lookup.  key = 
pub fn create_field_id_lookup(category_lookup: &HashMap<String, i32>, client: &Client, url: String) -> HashMap<String, i32> {
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

// Create field value id lookup. key = field_id + "|" + value, returns field_value_id
pub fn create_field_value_id_lookup(field_id_lookup: &HashMap<String, i32>, client: &Client, url: String) -> HashMap<String, i32> {
    let mut field_value_id_lookup: HashMap<String, i32> = HashMap::new();
    for field in field_id_lookup {
        let url = url.to_string() + field.1.to_string().as_str();
        // println!("field_values_by_field_id_url: {}", url);
        let field_values = get_field_values_for_field_id(client, url);
        for field_value in field_values {
            let key = field.1.to_string().as_str().to_owned() + "|" + field_value.value.as_str();
            field_value_id_lookup.insert(key, field_value.field_value_id);
        }
    }
    field_value_id_lookup
}

pub fn create_product_id_lookup() -> HashMap<String, i32> {
    println!("env path: {:?}", env::current_dir());
    let file = File::open("data/ProductLookup.csv").expect("Did not find file data/ProductLookup.csv");
    let mut reader = csv::Reader::from_reader(file);

    let mut product_lookup = HashMap::new();
    for line in reader.deserialize() {
        let record: ProductLookup = line.unwrap();
        product_lookup.insert(record.part_number.clone(), record.product_id.clone());
    }
    product_lookup
}

pub fn create_sku_id_lookup() -> HashMap<String, i32> {
    let file = File::open("data/SkuLookup.csv").expect("data/SkuLookup.csv");
    let mut reader = csv::Reader::from_reader(file);

    let mut sku_lookup = HashMap::new();
    for line in reader.deserialize() {
        let record: SkuLookup = line.unwrap();
        sku_lookup.insert(record.part_number.clone(), record.sku_id.clone());
    }
    sku_lookup
}

#[cfg(test)]
mod tests {
    use vtex::model::Brand;

    #[test]
    fn find_brand_id() {
        let brands = [Brand::new(Some(1), "Nike".to_string(), Some("Nike".to_string()), Some("Nike".to_string()), Some("Nike".to_string()), true, None, None, None, None),
            Brand::new(Some(2), "Adidas".to_string(), Some("Adidas".to_string()), Some("Adidas".to_string()), Some("Adidas".to_string()), true, None, None, None, None),
            Brand::new(Some(3), "New Balance".to_string(), Some("New Balance".to_string()), Some("New Balance".to_string()), Some("New Balance".to_string()), true, None, None, None, None),
            Brand::new(Some(4), "Saucony".to_string(), Some("Saucony".to_string()), Some("Saucony".to_string()), Some("Saucony".to_string()), true, None, None, None, None)
        ];

        let brand_id: Vec<Option<i32>> = brands
        .iter()
        .filter(|b| b.name.eq("New Balance"))
        .map(|b| b.id)
        .collect();
        println!("brand_id: {:?}", brand_id[0].unwrap());
    }

}