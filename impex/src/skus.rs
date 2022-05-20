use futures::{executor::block_on, stream, StreamExt};
use governor::{Jitter, Quota, RateLimiter};
use log::*;
use reqwest::{Client, StatusCode};
use vtex::{model::Sku, utils};

use std::{
    collections::HashMap, error::Error, fs::File, num::NonZeroU32, sync::Arc, time::Duration,
};

pub async fn load_skus(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    rate_limit: NonZeroU32,
) -> Result<(), Box<dyn Error>> {
    info!("Starting SKU load");
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/stockkeepingunit"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut sku_recs: Vec<Sku> = Vec::new();

    info!("Start: Reading input file to ensure values can be parsed");
    let mut e = 0;
    for line in rdr.deserialize() {
        match line {
            Ok(record) => {
                let sku_rec: Sku = record;
                sku_recs.push(sku_rec);
            }
            Err(err) => {
                error!("Error parsing row: {:?}", err);
                e += 1;
            }
        }
    }
    info!("Finished: Reading input file");
    info!(
        "Records successfully read: {}. Records not read (errors): {}",
        sku_recs.len(),
        e
    );

    // After full file read and removing non-deserialized records
    info!("Start: Looking up ProductId if not passed in the file");
    let mut product_lookup: HashMap<String, i32> = HashMap::new();
    let mut sku_recs_with_product_id: Vec<Sku> = Vec::new();
    for mut line in sku_recs {
        debug!("sku_record: {:?}", line);
        if line.product_id.is_none() {
            debug!("line.product_id was none");
            if !product_lookup.contains_key(&line.product_ref_id) {
                let get_product_id = utils::get_product_by_ref_id(
                    &line.product_ref_id,
                    client,
                    &account_name,
                    &environment,
                )
                .await;
                match get_product_id {
                    Ok(product_id) => {
                        product_lookup.insert(line.product_ref_id.clone(), product_id);
                        line.product_id = Some(product_id);
                    }
                    Err(err) => {
                        error!("Error: SKU record will be skipped: {}", err);
                    }
                }
            } else {
                debug!(
                    "product_lookup hit. product_ref_id: {} found.",
                    line.product_ref_id
                );
                line.product_id = Some(*product_lookup.get(&line.product_ref_id).unwrap());
            }
        }
        sku_recs_with_product_id.push(line);
    }
    info!("Finished: Looking up ProductId if not passed in the file");
    debug!("sku_recs length: {}", sku_recs_with_product_id.len());

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));

    let bodies = stream::iter(sku_recs_with_product_id)
        .map(|record| {
            let client = &client;
            let url = &url;
            let lim = Arc::clone(&lim);
            async move {
                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));

                debug!("sku record: {:?}", record);

                let response = client.post(url).json(&record).send().await?;

                info!(
                    "sku: {:?}: response: {:?}",
                    record.ref_id,
                    response.status()
                );
                if response.status() == StatusCode::TOO_MANY_REQUESTS {
                    info!("headers: {:?}", response.headers());
                }
                response.text().await
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

    info!("finished SKU load");

    Ok(())
}

pub async fn update_skus(
    file_path: String,
    client: &Client,
    account_name: String,
    environment: String,
    concurrent_requests: usize,
    rate_limit: NonZeroU32,
) -> Result<(), Box<dyn Error>> {
    info!("Starting SKU update");
    let url = "https://{accountName}.{environment}.com.br/api/catalog/pvt/stockkeepingunit/{skuId}"
        .replace("{accountName}", &account_name)
        .replace("{environment}", &environment);

    let input = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(input);

    let mut sku_recs: Vec<Sku> = Vec::new();

    info!("Start: Reading input file to ensure values can be parsed");
    let mut e = 0;
    for line in rdr.deserialize() {
        match line {
            Ok(record) => {
                let sku_rec: Sku = record;
                sku_recs.push(sku_rec);
            }
            Err(err) => {
                error!("Error parsing row: {:?}", err);
                e += 1;
            }
        }
    }
    info!("Finished: Reading input file");
    info!(
        "Records successfully read: {}. Records not read (errors): {}",
        sku_recs.len(),
        e
    );

    // After full file read and removing non-deserialized records
    info!("Start: Looking up ProductId if not passed in the file");
    let mut product_lookup: HashMap<String, i32> = HashMap::new();
    let mut sku_recs_with_product_id: Vec<Sku> = Vec::new();
    for mut line in sku_recs {
        debug!("sku_record: {:?}", line);
        if line.product_id.is_none() {
            debug!("line.product_id was none");
            if !product_lookup.contains_key(&line.product_ref_id) {
                let get_product_id = utils::get_product_by_ref_id(
                    &line.product_ref_id,
                    client,
                    &account_name,
                    &environment,
                )
                .await;
                match get_product_id {
                    Ok(product_id) => {
                        product_lookup.insert(line.product_ref_id.clone(), product_id);
                        line.product_id = Some(product_id);
                    }
                    Err(err) => {
                        error!("Error: SKU record will be skipped: {}", err);
                    }
                }
            } else {
                debug!(
                    "product_lookup hit. product_ref_id: {} found.",
                    line.product_ref_id
                );
                line.product_id = Some(*product_lookup.get(&line.product_ref_id).unwrap());
            }
        }
        sku_recs_with_product_id.push(line);
    }
    info!("Finished: Looking up ProductId if not passed in the file");
    debug!("sku_recs length: {}", sku_recs_with_product_id.len());

    let lim = Arc::new(RateLimiter::direct(Quota::per_second(rate_limit)));

    let bodies = stream::iter(sku_recs_with_product_id)
        .map(|record| {
            let client = &client;
            let url = &url;
            let lim = Arc::clone(&lim);
            async move {
                block_on(lim.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100))));
                let url_with_sku_id =
                    url.replace("{skuId}", record.id.unwrap().to_string().as_str());
                debug!("sku record: {:?}", record);

                let response = client.put(&url_with_sku_id).json(&record).send().await?;

                info!(
                    "sku: {:?}: response: {:?}",
                    record.ref_id,
                    response.status()
                );
                if response.status() == StatusCode::TOO_MANY_REQUESTS {
                    info!("headers: {:?}", response.headers());
                }
                response.text().await
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

    info!("finished SKU load");

    Ok(())
}

pub async fn count_skus(
    client: &Client,
    account_name: String,
    environment: String,
) -> Result<(), Box<dyn Error>> {
    info!("Starting SKU Count");
    utils::get_all_sku_ids(client, &account_name, &environment).await;
    info!("Finished SKU Count");

    Ok(())
}
