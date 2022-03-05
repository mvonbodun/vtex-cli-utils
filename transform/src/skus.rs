use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use vtex::model::{Sku};

use crate::csvrecords::{ ProdHeaderRecord, SkuDefineAttrValueRecord };

use crate::utils;

// Create a struct to hold the SKU name
#[derive(Debug, Serialize, Deserialize)]
struct SkuName {
    part_number: String,
    sku_name: String,
}

fn create_attr_value_lookup() -> HashMap<String, String> {
    let in_file = File::open("transform/data/in/SKUDefiningAttributeValue-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let mut sku_name_lookup: HashMap<String, String> = HashMap::new();
    for line in reader.deserialize() {
        let record: SkuDefineAttrValueRecord = line.unwrap();
        let sku_name: String;
        if sku_name_lookup.contains_key(&record.part_number) {
            let name = sku_name_lookup.get(&record.part_number).unwrap();
            sku_name = name.as_str().to_owned() + " " + record.name.as_str() + " " + record.value.as_str().trim();
            // sku_name_lookup.insert(record.part_number, name.as_str().to_owned() + " " + record.name.as_str() + " " + record.value.as_str().trim());

        } else {
            sku_name = record.name + " " + record.value.trim();
            // sku_name_lookup.insert(record.part_number, record.name + " " + record.value.trim());
        }
        // println!("sku_name: {}", sku_name);
        sku_name_lookup.insert(record.part_number, sku_name);
    }
    sku_name_lookup    
}

pub fn build_sku_file() -> Result<(), Box<dyn Error>> {

    // Build the PartNumber to ProductId lookup
    let product_lookup = utils::create_product_id_lookup();
    
    // Build the Attr Value lookup to create the SKU name
    let sku_name_lookup = create_attr_value_lookup();

    // Setup the input and output files
    let in_file = File::open("transform/data/in/ProductHeaderItem-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = "transform/data/out/Skus.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    // println!("product_lookup: {:?}", product_lookup);

    // Process the input file
    let mut x = 0;
    for line in reader.deserialize() {
        let record: ProdHeaderRecord = line.unwrap();
        let product_id = product_lookup.get(&record.parent_part_number).unwrap().to_owned();
        // println!("PartNumber: {} ProductId: {}", &record.parent_part_number, product_id);
        let image_url = "https://images.beallsflorida.com/i/beallsflorida/".to_owned() + record.ipsid.as_str() + "-yyy?w=1000&h=1000&fmt=auto&qlt=default&img404=404&v=1";

        let sku = Sku {
            id: None,
            product_id: Some(product_id),
            product_ref_id: "".to_string(),
            is_active: Some(false),
            name: sku_name_lookup.get(&record.part_number).unwrap().to_string(),
            ref_id: record.part_number.clone(),
            image_url: Some(image_url),
            packaged_height: 0.0,
            packaged_length: 0.0,
            packaged_width: 0.0,
            packaged_weight_kg: 0.0,
            height: Some(0.0),
            length: Some(0.0),
            width: Some(0.0),
            weight_kg: Some(0.0),
            cubic_weight: Some(0.0),
            is_kit: Some(false),
            creation_date: Some("2021-10-01T00:00:00".to_string()),
            reward_value: None,
            estimated_date_arrival: None,
            manufacturer_code: None,
            commercial_condition_id: None,
            measurement_unit: Some("un".to_string()),
            unit_multiplier: Some(1.0),
            modal_type: None,
            kit_itens_sell_apart: Some(false),
            activate_if_possible: Some(true),
        };
        writer.serialize(sku)?;
        x = x + 1;
    }
    // Flush the records
    writer.flush()?;
    println!("records written: {}", x);

    Ok(())
}

#[cfg(test)]
mod test {

    use crate::utils::create_product_id_lookup;

    #[test]
    fn test_key_lookup() {
        let product_lookup = create_product_id_lookup();
        let product_id = product_lookup.get("P000222475").unwrap().to_owned();
        println!("product_id: {}", product_id);
    }
}