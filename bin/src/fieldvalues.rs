use reqwest::{Client, StatusCode};
use std::error::Error;
use std::fs::File;

use vtex::model::SpecificationValue;

pub async fn load_field_values(file_path: String, client: &Client, url: String) -> Result<(), Box<dyn Error>> {

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    for line in rdr.deserialize() {
        let record: SpecificationValue = line?;
        println!("SpecificationValue Record: {:?}", record);

        let response = client
            .post(&url)
            .json(&record)
            .send()
            .await?;
            
        match response.status() {
            StatusCode::OK => {
                let result: SpecificationValue = response.json().await?;
                println!("FieldValueId: {:?}", result.field_value_id);
            },
            StatusCode::BAD_REQUEST => {
                println!("Error: {:#?}", response.text().await?);
            },
            _ => {
                println!("Error: {:#?}", response.text().await?);
            },
        }
    }

    Ok(())
}
