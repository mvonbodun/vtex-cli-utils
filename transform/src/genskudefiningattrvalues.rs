use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

use crate::csvrecords::{SkuDefineAttrValueRecord, ProdHeaderRecord};

pub fn gen_attr_value_subset() -> Result<(), Box<dyn Error>> {

    // Read in the items that are in the subset
    // Setup the input and output files
    let item_file = File::open("transform/data/in/ProductHeaderItem-sorted-subset.csv").unwrap();
    let mut item_reader = csv::Reader::from_reader(item_file);
    let mut item_set: HashSet<String> = HashSet::new();
    for line in item_reader.deserialize() {
        let record: ProdHeaderRecord = line.unwrap();
        item_set.insert(record.part_number);
    }
    println!("item_set length: {:?}", item_set);

    // Read in the input file
    let in_file = File::open("transform/data/in/SKUDefiningAttributeValueOriginal.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = "transform/data/in/SKUDefiningAttributeValue-sorted-subset.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut x = 0;
    for line in reader.deserialize() {
        let record: SkuDefineAttrValueRecord = line.unwrap();
        if item_set.contains(&record.part_number) {
            writer.serialize(record)?;
            x = x + 1;
        }
    }
    writer.flush()?;
    println!("records writen: {}", x);

    Ok(())
}