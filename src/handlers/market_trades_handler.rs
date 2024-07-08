use anyhow::Result;
use tokio::sync::watch;
use std::time::Duration;
use crate::api::market_trades::fetch_market_trades;
use crate::prompt_for_product_id;
use tokio::time;

pub async fn fetch_market_trades_handler(mut stop: watch::Receiver<()>) -> Result<()> {
    let product_id = prompt_for_product_id();
    loop {
        tokio::select! {
            _ = stop.changed() => {
                println!("Stopping fetch_market_trades_handler");
                break;
            }
            _ = time::sleep(Duration::from_secs(1)) => {
                match fetch_market_trades(&product_id).await {
                    Ok(market_trades) => println!("Market Trades for {}: {:?}", product_id, market_trades),
                    Err(e) => println!("Error fetching market trades for {}: {:?}", product_id, e),
                }
            }
        }
    }
    Ok(())
}
