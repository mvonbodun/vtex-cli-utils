use futures::{stream, StreamExt};
use log::*;
use reqwest::{Client, StatusCode};
use std::error::Error;
use std::fs::File;
use vtex::model::SimilarCategory;

pub async fn load_similar_categories(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
) -> Result<(), Box<dyn Error>> {
    info!("Starting Similar Categories load");
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/product/{productId}/similarcategory/{categoryId}"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);
    let input = File::open(&file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut simcat_recs: Vec<SimilarCategory> = Vec::new();

    for line in rdr.deserialize() {
        let record: SimilarCategory = line?;
        simcat_recs.push(record);
    }
    info!(
        "{} records read from input file: {}",
        simcat_recs.len(),
        file_path
    );

    // let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));
    // let mut bodies = stream::iter(simcat_recs).ratelimit_stream(&lim);
    let bodies = stream::iter(simcat_recs)
        .map(|record| {
            let client = &client;
            let url = &url;
            // let lim = Arc::clone(&lim);
            async move {
                // block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));
                let url = url.replace("{productId}", record.product_id.to_string().as_str());
                let url = url.replace("{categoryId}", record.category_id.to_string().as_str());

                let response = client.post(&url).json(&record).send().await?;

                let status = response.status();
                info!(
                    "product: {:?}: category: {:?}: response: {:?}",
                    record.product_id, record.category_id, status
                );
                let text = response.text().await;
                if status != StatusCode::OK {
                    info!("text: {:?}", text);
                }
                text
            }
        })
        .buffer_unordered(concurrent_requests);
    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => info!("output: {:?}", b),
                Err(e) => error!("error: {:?}", e),
            }
        })
        .await;

    info!("finished Similar Categories load");

    Ok(())
}
