mod api;
mod models;
mod utils;

use api::{
    candles::fetch_candles,
    market_trades::fetch_market_trades,
    product_book::fetch_product_book,
    products::fetch_products,
    server_time::fetch_server_time,
};
use tokio::main;

#[main]
async fn main() -> Result<(), reqwest::Error> {
    // Fetch and print all products
    match fetch_products().await {
        Ok(products) => println!("Products: {:?}", products),
        Err(e) => println!("Error fetching products: {:?}", e),
    }

    // Fetch and print server time
    match fetch_server_time().await {
        Ok(server_time) => println!("Server Time: {:?}", server_time),
        Err(e) => println!("Error fetching server time: {:?}", e),
    }

    // Fetch and print order book for a specific product
    let product_id = "BTC-USD";
    match fetch_product_book(product_id).await {
        Ok(product_book) => println!("Product Book for {}: {:?}", product_id, product_book),
        Err(e) => println!("Error fetching product book for {}: {:?}", product_id, e),
    }

    // Fetch and print candles for a specific product
    let start = "2022-01-01T00:00:00Z";
    let end = "2022-01-02T00:00:00Z";
    let granularity = "ONE_HOUR";
    match fetch_candles(product_id, start, end, granularity).await {
        Ok(candles) => println!("Candles for {}: {:?}", product_id, candles),
        Err(e) => println!("Error fetching candles for {}: {:?}", product_id, e),
    }

    // Fetch and print market trades for a specific product
    match fetch_market_trades(product_id).await {
        Ok(market_trades) => println!("Market Trades for {}: {:?}", product_id, market_trades),
        Err(e) => println!("Error fetching market trades for {}: {:?}", product_id, e),
    }

    Ok(())
}
