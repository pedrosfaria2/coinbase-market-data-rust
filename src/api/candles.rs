use crate::models::{Candle, CandlesResponse}; // Importing necessary models.
use crate::utils::BASE_URL; // Importing the base URL for API requests.
use anyhow::Result; // Importing Result type for error handling.
use chrono::{DateTime, Utc}; // Importing DateTime and Utc types for date and time handling.

// Asynchronously fetches candle data for a specific product within a given time range and granularity.
pub async fn fetch_candles(
    product_id: &str,  // The ID of the product to fetch data for.
    start: &str,       // Start time in ISO 8601 format.
    end: &str,         // End time in ISO 8601 format.
    granularity: &str, // The granularity of the candles.
) -> Result<Vec<Candle>> {
    // Returns a Result with a vector of Candle objects or an error.

    // Convert start and end ISO 8601 date strings to DateTime<Utc> objects.
    let start_dt: DateTime<Utc> = start.parse()?; // Parsing start time.
    let end_dt: DateTime<Utc> = end.parse()?; // Parsing end time.
    let start_ts = start_dt.timestamp(); // Converting start time to Unix timestamp.
    let end_ts = end_dt.timestamp(); // Converting end time to Unix timestamp.

    // Constructing the API request URL.
    let url = format!(
        "{}/market/products/{}/candles?start={}&end={}&granularity={}",
        BASE_URL, product_id, start_ts, end_ts, granularity
    );

    // Sending the GET request to the API.
    let response = reqwest::get(&url).await?;

    // Attempting to deserialize the JSON response into CandlesResponse.
    let candles_response: CandlesResponse = response.json().await?;

    // Returning the candles data.
    Ok(candles_response.candles)
}
