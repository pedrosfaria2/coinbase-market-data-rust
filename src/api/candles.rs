use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::models::{CandlesResponse, Candle};
use crate::utils::BASE_URL;

pub async fn fetch_candles(product_id: &str, start: &str, end: &str, granularity: &str) -> Result<Vec<Candle>> {
    // Convert ISO date strings to Unix timestamps
    let start_dt: DateTime<Utc> = start.parse()?;
    let end_dt: DateTime<Utc> = end.parse()?;
    let start_ts = start_dt.timestamp();
    let end_ts = end_dt.timestamp();

    let url = format!(
        "{}/market/products/{}/candles?start={}&end={}&granularity={}",
        BASE_URL, product_id, start_ts, end_ts, granularity
    );
    let response = reqwest::get(&url).await?;

    // Attempt to deserialize the JSON response
    let candles_response: CandlesResponse = response.json().await?;
    Ok(candles_response.candles)
}
