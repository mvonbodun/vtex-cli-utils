use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use regex::Regex;
use vtex::model::SkuFile;

use crate::csvrecords::ProdHeaderRecord;
use crate::utils;

pub fn build_sku_file() -> Result<(), Box<dyn Error>> {

    // Build a Sku_id lookup fn
    let sku_id_lookup = utils::create_sku_id_lookup();

    // Setup the input and output files
    let in_file = File::open("transform/data/in/ProductHeaderItem-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = "transform/data/out/SkuFiles.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    // Create HashSet to track if this is the first time the part_number appears
    let mut part_number_set: HashSet<String> = HashSet::new();
    // Regex to make the name friendly
    let re = Regex::new(r"([^\w\s-])").unwrap();

    let mut x = 0;
    for line in reader.deserialize() {
        let record: ProdHeaderRecord = line?;
        let is_main: bool;
        if part_number_set.contains(&record.parent_part_number) {
            is_main = false;
        } else {
            is_main = true;
        }
        let url = "https://images.beallsflorida.com/i/beallsflorida/".to_owned() + record.ipsid.as_str() + "-yyy?w=1000&h=1000&fmt=auto&qlt=default&img404=404&v=1";
        // Remove special characters from the name
        let name = record.name.replace(" ", "-");
        let name = re.replace_all(&name, "");
        if sku_id_lookup.contains_key(&record.part_number) {
            let sku_file = SkuFile {
                id: None,
                sku_id: *sku_id_lookup.get(&record.part_number).unwrap(),
                is_main: Some(is_main),
                archive_id: None,
                name: Some(name.to_string()),
                label: Some(name.to_string()),
                url: Some(url),
            };
            writer.serialize(sku_file)?;
            part_number_set.insert(record.parent_part_number);
            x = x + 1;
        }
    
    }
    // Flush the records
    writer.flush()?;
    println!("records writtern: {}", x);

    Ok(())
}

#[cfg(test)]
mod tests {
    use regex::{self, Regex};

    #[test]
    fn regex_text() {
        let name = String::from("A.-Byer-Shutter?-Crochet,-Back-Tank-Top");
        let re = Regex::new(r"([^\w\s-])").unwrap();
        // let re = Regex::new(r"(^.*?://)").unwrap();
        let after = re.replace_all(&name, "");
        println!("after: {}", after);
    }
}
