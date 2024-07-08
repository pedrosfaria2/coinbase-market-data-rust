use anyhow::Result;
use tokio::sync::watch;
use std::time::Duration;
use crate::api::candles::fetch_candles;
use crate::prompt_for_product_id;
use tokio::time;

pub async fn fetch_candles_handler(mut stop: watch::Receiver<()>) -> Result<()> {
    let product_id = prompt_for_product_id();
    let start = "2022-01-01T00:00:00Z";
    let end = "2022-01-02T00:00:00Z";
    let granularity = "ONE_HOUR";
    loop {
        tokio::select! {
            _ = stop.changed() => {
                println!("Stopping fetch_candles_handler");
                break;
            }
            _ = time::sleep(Duration::from_secs(1)) => {
                match fetch_candles(&product_id, start, end, granularity).await {
                    Ok(candles) => println!("Candles for {}: {:?}", product_id, candles),
                    Err(e) => println!("Error fetching candles for {}: {:?}", product_id, e),
                }
            }
        }
    }
    Ok(())
}
