use std::error::Error;
use std::fs::File;
use vtex::model::Price;

use crate::csvrecords::PricingRecord;
use crate::utils;

pub fn build_price_file() -> Result<(), Box<dyn Error>> {

    // Build a Sku_id lookup fn
    let sku_id_lookup = utils::create_sku_id_lookup();
    // Setup the input and output files
    let in_file = File::open("transform/data/in/Pricing-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = "transform/data/out/Prices.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut x = 0;
    for line in reader.deserialize() {
        let record: PricingRecord = line.unwrap();
        // println!("record: {:?}", record);

        if sku_id_lookup.contains_key(&record.catentry_part_number) {
            let price = Price {
                sku_id: Some(*sku_id_lookup.get(&record.catentry_part_number).unwrap()),
                ref_id: record.catentry_part_number.clone(),
                base_price: Some(record.price),
                cost_price: Some(record.price),
                list_price: Some(record.price),
                markup: None,
            };
            writer.serialize(price)?;
            x = x + 1;
        }
    }
    // Flush the records
    writer.flush()?;
    println!("records written: {}", x);
    
    Ok(())
}