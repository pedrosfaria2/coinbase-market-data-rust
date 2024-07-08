use serde::Deserialize; // Import the Deserialize trait for deserializing JSON data.
use std::option::Option; // Import Option type.
use std::vec::Vec; // Import Vec type.

// Struct to represent the server time.
#[derive(Deserialize, Debug)]
pub struct ServerTime {
    pub iso: String, // ISO formatted date-time string.
    #[serde(rename = "epochSeconds")]
    pub epoch_seconds: String, // Epoch time in seconds as a string.
    #[serde(rename = "epochMillis")]
    pub epoch_millis: String, // Epoch time in milliseconds as a string.
}

// Struct to represent an entry in the product book (bid or ask).
#[derive(Deserialize, Debug)]
pub struct ProductBookEntry {
    pub price: String, // Price of the bid or ask.
    pub size: String,  // Size of the bid or ask.
}

// Struct to represent the price book for a product, including bids and asks.
#[derive(Deserialize, Debug)]
pub struct ProductBookPriceBook {
    pub product_id: String,          // ID of the product.
    pub bids: Vec<ProductBookEntry>, // List of bids.
    pub asks: Vec<ProductBookEntry>, // List of asks.
    pub time: String,                // Timestamp of the price book.
}

// Struct to represent the product book, containing the price book.
#[derive(Deserialize, Debug)]
pub struct ProductBook {
    pub pricebook: ProductBookPriceBook, // Nested price book.
}

// Struct to represent the trading session details for FCM.
#[derive(Deserialize, Debug)]
pub struct FCMTradingSessionDetails {
    pub is_session_open: Option<bool>, // Indicates if the session is open.
    pub open_time: Option<String>,     // Opening time of the session.
    pub close_time: Option<String>,    // Closing time of the session.
}

// Struct to represent the details of perpetual contracts.
#[derive(Deserialize, Debug)]
pub struct PerpetualDetails {
    pub open_interest: Option<String>, // Open interest for perpetual contracts.
    pub funding_rate: Option<String>,  // Funding rate for perpetual contracts.
    pub funding_time: Option<String>,  // Funding time for perpetual contracts.
}

// Struct to represent the details of future products.
#[derive(Deserialize, Debug)]
pub struct FutureProductDetails {
    pub venue: Option<String>,              // Venue of the future product.
    pub contract_code: Option<String>,      // Contract code of the future product.
    pub contract_expiry: Option<String>,    // Expiry date of the contract.
    pub contract_size: Option<String>,      // Size of the contract.
    pub contract_root_unit: Option<String>, // Root unit of the contract.
    pub group_description: Option<String>,  // Description of the group.
    pub contract_expiry_timezone: Option<String>, // Timezone of the contract expiry.
    pub group_short_description: Option<String>, // Short description of the group.
    pub risk_managed_by: Option<String>,    // Entity managing the risk.
    pub contract_expiry_type: Option<String>, // Type of contract expiry.
    pub perpetual_details: Option<PerpetualDetails>, // Perpetual details (if any).
    pub contract_display_name: Option<String>, // Display name of the contract.
}

// Struct to represent a product with detailed information.
#[derive(Deserialize, Debug)]
pub struct Product {
    pub product_id: String,                                     // Product ID.
    pub price: String,                                          // Current price.
    pub price_percentage_change_24h: String,                    // 24h price change percentage.
    pub volume_24h: String,                                     // 24h trading volume.
    pub volume_percentage_change_24h: String,                   // 24h volume change percentage.
    pub base_increment: String,                                 // Increment for the base currency.
    pub quote_increment: String,                                // Increment for the quote currency.
    pub quote_min_size: String,                                 // Minimum quote size.
    pub quote_max_size: String,                                 // Maximum quote size.
    pub base_min_size: String,                                  // Minimum base size.
    pub base_max_size: String,                                  // Maximum base size.
    pub base_name: String,                                      // Base currency name.
    pub quote_name: String,                                     // Quote currency name.
    pub watched: bool,             // Indicates if the product is watched.
    pub is_disabled: bool,         // Indicates if the product is disabled.
    pub new: bool,                 // Indicates if the product is new.
    pub status: String,            // Status of the product.
    pub cancel_only: bool,         // Indicates if only cancel orders are allowed.
    pub limit_only: bool,          // Indicates if only limit orders are allowed.
    pub post_only: bool,           // Indicates if only post-only orders are allowed.
    pub trading_disabled: bool,    // Indicates if trading is disabled.
    pub auction_mode: bool,        // Indicates if auction mode is active.
    pub product_type: String,      // Type of the product.
    pub quote_currency_id: String, // Quote currency ID.
    pub base_currency_id: String,  // Base currency ID.
    pub fcm_trading_session_details: Option<serde_json::Value>, // FCM trading session details.
    pub mid_market_price: String,  // Mid-market price.
    pub alias: String,             // Alias of the product.
    pub alias_to: Vec<String>,     // Aliases to other products.
    pub base_display_symbol: String, // Display symbol for the base currency.
    pub quote_display_symbol: String, // Display symbol for the quote currency.
    pub view_only: bool,           // Indicates if the product is view-only.
    pub price_increment: String,   // Price increment.
    pub display_name: String,      // Display name of the product.
    pub product_venue: String,     // Venue of the product.
    pub approximate_quote_24h_volume: String, // Approximate 24h quote volume.
}

// Struct to represent a response containing a list of products.
#[derive(Deserialize, Debug)]
pub struct ProductsResponse {
    pub products: Vec<Product>, // List of products.
}

// Struct to represent a single candle (OHLC data).
#[derive(Deserialize, Debug)]
pub struct Candle {
    pub start: String,  // Start time of the candle.
    pub low: String,    // Low price during the candle period.
    pub high: String,   // High price during the candle period.
    pub open: String,   // Opening price of the candle.
    pub close: String,  // Closing price of the candle.
    pub volume: String, // Volume during the candle period.
}

// Struct to represent a response containing a list of candles.
#[derive(Deserialize, Debug)]
pub struct CandlesResponse {
    pub candles: Vec<Candle>, // List of candles.
}

// Struct to represent a market trade.
#[derive(Deserialize, Debug)]
pub struct MarketTrade {
    pub trade_id: String,    // ID of the trade.
    pub product_id: String,  // ID of the product.
    pub price: String,       // Price at which the trade occurred.
    pub size: String,        // Size of the trade.
    pub time: String,        // Time at which the trade occurred.
    pub side: String,        // Side of the trade (buy/sell).
    pub bid: Option<String>, // Bid price at the time of the trade (optional).
    pub ask: Option<String>, // Ask price at the time of the trade (optional).
}

// Struct to represent a response containing a list of market trades.
#[derive(Deserialize, Debug)]
pub struct MarketTradesResponse {
    pub trades: Vec<MarketTrade>, // List of market trades.
}
