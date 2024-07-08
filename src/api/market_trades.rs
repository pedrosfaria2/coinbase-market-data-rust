use anyhow::Result;
use crate::models::{MarketTradesResponse, MarketTrade};
use crate::utils::BASE_URL;

pub async fn fetch_market_trades(product_id: &str) -> Result<Vec<MarketTrade>> {
    let url = format!("{}/market/products/{}/ticker", BASE_URL, product_id);
    let response = reqwest::get(&url).await?;

    // Attempt to deserialize the JSON response
    let market_trades_response: MarketTradesResponse = response.json().await?;
    Ok(market_trades_response.trades)
}
