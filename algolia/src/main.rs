use log::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    algolia::setup();
    info!("Start");
    algolia::run().await?;
    info!("Done");

    Ok(())
}
