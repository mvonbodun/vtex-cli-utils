use log::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    algolia::setup();
    info!("Starting Algolia Index Build");
    algolia::test_rate_limit().await?;
    info!("Finished Algolia Index Build");

    Ok(())
}
