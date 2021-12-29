use std::error::Error;
use std::fs::File;

use vtex::model::Inventory;
use crate::csvrecords::InventoryRecord;
use crate::utils;

pub fn build_inventory_file() -> Result<(), Box<dyn Error>> {

    // Build a Sku_id lookup fn
    let sku_id_lookup = utils::create_sku_id_lookup();
    // println!("sku_id_lookup: {:?}", sku_id_lookup);
    // Setup the input and output files
    let in_file = File::open("transform/data/in/Inventory-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = "transform/data/out/Inventory.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    let mut x = 0;
    for line in reader.deserialize() {
        let record: InventoryRecord = line.unwrap();
        // println!("record: {:?}", record);

        if sku_id_lookup.contains_key(&record.part_number) {
            let inventory = Inventory {
                warehouse_id: "warehouse1".to_owned(),
                sku_id: *sku_id_lookup.get(&record.part_number).unwrap(),
                unlimited_quantity: false,
                date_utc_on_balance_system: None,
                quantity: record.quantity,
            };
            writer.serialize(inventory)?;
            x = x + 1;
        }
    }
    // Flush the records
    writer.flush()?;
    println!("records written: {}", x);

    Ok(())
}