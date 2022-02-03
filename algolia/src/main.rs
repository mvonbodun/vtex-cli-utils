use log::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    vtex_algolia::setup();
    info!("Starting Algolia Index Build");
    vtex_algolia::run().await?;
    info!("Finished Algolia Index Build");

    Ok(())
}
