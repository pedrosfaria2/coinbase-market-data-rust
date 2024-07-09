use crate::api::market_trades::fetch_market_trades; // Importing the fetch_market_trades function.
use crate::models::MarketTrade; // Importing the MarketTrade model.
use anyhow::Result; // Importing the Result type from anyhow for error handling.
use std::time::Duration; // Importing Duration for time handling.
use tokio::sync::watch; // Importing watch from tokio for synchronization.
use tokio::time; // Importing time module from tokio for sleep functionality.

// Asynchronously fetches and displays market trades for a specific product.
pub async fn fetch_market_trades_handler(
    mut stop: watch::Receiver<()>,
    product_id: String,
) -> Result<()> {
    loop {
        // Using tokio::select! to handle concurrent tasks.
        tokio::select! {
            // Checking if the stop signal has been received.
            _ = stop.changed() => {
                println!("Stopping fetch_market_trades_handler"); // Print message when stopping.
                break; // Exit the loop.
            }
            // Adding a delay of 50 milliseconds before the next fetch.
            _ = time::sleep(Duration::from_millis(300)) => {
                // Fetching market trades and handling the result.
                match fetch_market_trades(&product_id).await {
                    Ok(mut market_trades) => {
                        // Sort trades by time using a stable sort to maintain order for equal elements
                        market_trades.sort_unstable_by(|a, b| a.time.cmp(&b.time));
                        // Limit the number of trades to display
                        let trades_to_display = market_trades.iter().take(30).collect::<Vec<_>>();
                        // Move cursor to the specific location for this handler
                        print!("\x1B[1;1H");
                        // Display the trades
                        display_market_trades(&trades_to_display); // Displaying market trades.
                    },
                    Err(e) => println!("Error fetching market trades for {}: {:?}", product_id, e), // Handling fetch error.
                }
            }
        }
    }
    Ok(()) // Returning Ok result.
}

// Function to display the market trades in a table format.
fn display_market_trades(trades: &[&MarketTrade]) {
    // Print header at the top of the designated area
    println!(
        "{}{:<15} {:<10} {:<10} {:<10} {:<25} {:<5}",
        format!("\x1B[1;1H"),
        "Trade ID",
        "Product ID",
        "Price",
        "Size",
        "Time",
        "Side"
    );

    // Print each trade in the designated area
    for (i, trade) in trades.iter().enumerate() {
        println!(
            "{}{:<15} {:<10} {:<10} {:<10} {:<25} {:<5}",
            format!("\x1B[{};1H", i + 2),
            trade.trade_id,
            trade.product_id,
            trade.price,
            trade.size,
            trade.time,
            trade.side
        );
    }
}
