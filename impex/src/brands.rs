use futures::{stream, StreamExt};
use log::*;
use std::error::Error;
use std::fs::File;

use vtex::model::{Brand};
use reqwest::Client;

pub async fn load_brands(file_path: String, client: &Client, account_name: String, environment: String, concurrent_requests: usize) -> Result<(), Box<dyn Error>> {

    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/brand"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);
    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut brand_recs: Vec<Brand> = Vec::new();

    for line in rdr.deserialize() {
        let record: Brand = line?;
        brand_recs.push(record);
    }
    info!("brand records: {:?}", brand_recs.len());

    let bodies = stream::iter(brand_recs)
        .map(|record| {
            let client = &client;
            let url = &url;
            async move {
                let response = client
                .post(url)
                .json(&record)
                .send()
                .await?;

            info!("brand: {:?}: response: {:?}", record.id, response.status());

            response.json::<Brand>().await
  
            }
        })
        .buffer_unordered(concurrent_requests);
    bodies
        .for_each(|b| async {
            match b {
                Ok(_b) => (),
                Err(e) => error!("error: {:?}", e),
            }
        })
        .await;

        info!("Finished loading brands");
    Ok(())
}
