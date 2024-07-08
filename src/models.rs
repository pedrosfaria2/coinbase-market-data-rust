use serde::Deserialize;
use std::option::Option;
use std::vec::Vec;

#[derive(Deserialize, Debug)]
pub struct ServerTime {
    pub iso: String, // ISO formatted date-time string
    #[serde(rename = "epochSeconds")]
    pub epoch_seconds: String, // Epoch time in seconds as a string
    #[serde(rename = "epochMillis")]
    pub epoch_millis: String, // Epoch time in milliseconds as a string
}

#[derive(Deserialize, Debug)]
pub struct ProductBookEntry {
    pub price: String, // Price of the bid or ask
    pub size: String,  // Size of the bid or ask
}

#[derive(Deserialize, Debug)]
pub struct ProductBookPriceBook {
    pub product_id: String,          // ID of the product
    pub bids: Vec<ProductBookEntry>, // List of bids
    pub asks: Vec<ProductBookEntry>, // List of asks
    pub time: String,                // Timestamp of the price book
}

#[derive(Deserialize, Debug)]
pub struct ProductBook {
    pub pricebook: ProductBookPriceBook, // Nested Price Book
}

#[derive(Deserialize, Debug)]
pub struct FCMTradingSessionDetails {
    pub is_session_open: Option<bool>, // Indicates if the session is open
    pub open_time: Option<String>,     // Opening time of the session
    pub close_time: Option<String>,    // Closing time of the session
}

#[derive(Deserialize, Debug)]
pub struct PerpetualDetails {
    pub open_interest: Option<String>, // Open interest for perpetual contracts
    pub funding_rate: Option<String>,  // Funding rate for perpetual contracts
    pub funding_time: Option<String>,  // Funding time for perpetual contracts
}

#[derive(Deserialize, Debug)]
pub struct FutureProductDetails {
    pub venue: Option<String>,                    // Venue of the future product
    pub contract_code: Option<String>,            // Contract code of the future product
    pub contract_expiry: Option<String>,          // Expiry date of the contract
    pub contract_size: Option<String>,            // Size of the contract
    pub contract_root_unit: Option<String>,       // Root unit of the contract
    pub group_description: Option<String>,        // Description of the group
    pub contract_expiry_timezone: Option<String>, // Timezone of the contract expiry
    pub group_short_description: Option<String>,  // Short description of the group
    pub risk_managed_by: Option<String>,          // Entity managing the risk
    pub contract_expiry_type: Option<String>,     // Type of contract expiry
    pub perpetual_details: Option<PerpetualDetails>, // Perpetual details (if any)
    pub contract_display_name: Option<String>,    // Display name of the contract
}

#[derive(Deserialize, Debug)]
pub struct Product {
    pub product_id: String,
    pub price: String,
    pub price_percentage_change_24h: String,
    pub volume_24h: String,
    pub volume_percentage_change_24h: String,
    pub base_increment: String,
    pub quote_increment: String,
    pub quote_min_size: String,
    pub quote_max_size: String,
    pub base_min_size: String,
    pub base_max_size: String,
    pub base_name: String,
    pub quote_name: String,
    pub watched: bool,
    pub is_disabled: bool,
    pub new: bool,
    pub status: String,
    pub cancel_only: bool,
    pub limit_only: bool,
    pub post_only: bool,
    pub trading_disabled: bool,
    pub auction_mode: bool,
    pub product_type: String,
    pub quote_currency_id: String,
    pub base_currency_id: String,
    pub fcm_trading_session_details: Option<serde_json::Value>,
    pub mid_market_price: String,
    pub alias: String,
    pub alias_to: Vec<String>,
    pub base_display_symbol: String,
    pub quote_display_symbol: String,
    pub view_only: bool,
    pub price_increment: String,
    pub display_name: String,
    pub product_venue: String,
    pub approximate_quote_24h_volume: String,
}

#[derive(Deserialize, Debug)]
pub struct ProductsResponse {
    pub products: Vec<Product>,
}

#[derive(Deserialize, Debug)]
pub struct Candle {
    pub start: String,  // Start time of the candle
    pub low: String,    // Low price during the candle period
    pub high: String,   // High price during the candle period
    pub open: String,   // Opening price of the candle
    pub close: String,  // Closing price of the candle
    pub volume: String, // Volume during the candle period
}

#[derive(Deserialize, Debug)]
pub struct CandlesResponse {
    pub candles: Vec<Candle>,
}

#[derive(Deserialize, Debug)]
pub struct MarketTrade {
    pub trade_id: String,    // ID of the trade
    pub product_id: String,  // ID of the product
    pub price: String,       // Price at which the trade occurred
    pub size: String,        // Size of the trade
    pub time: String,        // Time at which the trade occurred
    pub side: String,        // Side of the trade (buy/sell)
    pub bid: Option<String>, // Bid price at the time of the trade (optional)
    pub ask: Option<String>, // Ask price at the time of the trade (optional)
}

#[derive(Deserialize, Debug)]
pub struct MarketTradesResponse {
    pub trades: Vec<MarketTrade>,
}
