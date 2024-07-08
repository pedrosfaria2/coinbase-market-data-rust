use crate::models::{MarketTrade, MarketTradesResponse}; // Importing necessary models.
use crate::utils::BASE_URL; // Importing the base URL for API requests.
use anyhow::Result; // Importing Result type for error handling.

// Asynchronously fetches market trades for a specific product.
pub async fn fetch_market_trades(product_id: &str) -> Result<Vec<MarketTrade>> {
    // Returns a Result with a vector of MarketTrade objects or an error.

    // Constructing the API request URL.
    let url = format!("{}/market/products/{}/ticker", BASE_URL, product_id);

    // Sending the GET request to the API.
    let response = reqwest::get(&url).await?;

    // Attempting to deserialize the JSON response into MarketTradesResponse.
    let market_trades_response: MarketTradesResponse = response.json().await?;

    // Returning the trades data.
    Ok(market_trades_response.trades)
}
