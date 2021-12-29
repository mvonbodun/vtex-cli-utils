use std::{error::Error, collections::HashSet};
use std::fs::File;
use vtex::model::SkuFile;

use crate::csvrecords::SkusInactive;

fn get_inactive_skus() -> HashSet<i32> {
    let in_file = File::open("transform/data/in/skus_inactive.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);

    let mut inactive_skus: HashSet<i32> = HashSet::new();
    for line in reader.deserialize() {
        let record: SkusInactive  = line.unwrap();
        inactive_skus.insert(record.sku_id);
    }
    inactive_skus
}

pub fn gen_inactive_sku_files() -> Result<(), Box<dyn Error>> {

    // Setup the input and output files
    let in_file = File::open("transform/data/out/SkuFiles.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = "transform/data/out/SkuFiles_inactive.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    let inactive_skus_lookup = get_inactive_skus();
    let mut x = 0;
    for line in reader.deserialize() {
        let record: SkuFile = line?;

        if inactive_skus_lookup.contains(&record.sku_id) {
            writer.serialize(record)?;
            x = x + 1;
        }

    }
    // Flush the records
    writer.flush()?;
    println!("Records written: {}", x);

    Ok(())
}