use vtex::model::Category;
use std::{error::Error, fs::File};

use crate::csvrecords::CatRecord;



pub fn build_category_file() -> Result<(), Box<dyn Error>> {
    
    // Setup the input and output files

    let in_file = File::open("transform/data/in/DeptCatalog-sorted-subset.csv").unwrap();
    let mut reader = csv::Reader::from_reader(in_file);
    let out_path = "transform/data/out/Categories.csv";
    let mut writer = csv::Writer::from_path(out_path)?;

    // Process the input file
    let mut x = 0;

    for line in reader.deserialize() {
        let record: CatRecord = line.unwrap();

        let category = Category {
            id: None,
            unique_identifier: Some(record.group_identifier.clone()),
            name: record.name.clone(),
            father_category_id: None,
            parent_unique_identifier: Some(record.parent_group_identifier.clone()),
            title: record.name.clone(),
            description: record.short_description.clone(),
            keywords: record.name.clone(),
            is_active: true,
            lomadee_campaign_code: None,
            ad_words_remarketing_code: None,
            show_in_store_front: true,
            show_brand_filter: true,
            active_store_front_link: true,
            global_category_id: None,
            stock_keeping_unit_selection_mode: "SPECIFICATION".to_string(),
            score: None,
            link_id: None,
            has_children: None
        };

        // Write the record
        writer.serialize(category)?;
        x = x + 1;
    }
    writer.flush()?;
    println!("records writtern: {}", x);

    Ok(())
}