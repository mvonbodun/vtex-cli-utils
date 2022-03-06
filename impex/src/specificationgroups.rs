use std::error::Error;
use std::fs::File;
use futures::{stream, StreamExt};
use log::*;
use reqwest::Client;
use vtex::model::SpecificationGroup;

pub async fn load_specification_groups(file_path: String, client: &Client, account_name: String, environment: String, concurrent_requests: usize) -> Result<(), Box<dyn Error>> {
    info!("Starting specification group load");
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/specificationgroup"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut specgroups_recs: Vec<SpecificationGroup> = Vec::new();

    for line in rdr.deserialize() {
        let record: SpecificationGroup = line?;
        specgroups_recs.push(record);
    }

    info!("specification group records: {:?}", specgroups_recs.len());

    let bodies = stream::iter(specgroups_recs)
        .map(|record| {
            let client = &client;
            let url = &url;
            async move {
                let response = client
                    .post(url)
                    .json(&record)
                    .send()
                    .await?;

                info!("specification group: {:?}: repsonse: {:?}", record.id, response.status());

                response.json::<SpecificationGroup>().await
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

        info!("Finished loading specification groups");

    Ok(())
}