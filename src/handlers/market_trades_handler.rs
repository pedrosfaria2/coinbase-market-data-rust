use crate::api::market_trades::fetch_market_trades; // Importing the fetch_market_trades function.
use crate::models::MarketTrade; // Importing the MarketTrade model.
use crate::prompt_for_product_id; // Importing the prompt_for_product_id function.
use anyhow::Result; // Importing the Result type from anyhow for error handling.
use prettytable::{format, Cell, Row, Table}; // Importing prettytable components for table formatting and display.
use std::time::Duration; // Importing Duration for time handling.
use tokio::sync::watch; // Importing watch from tokio for synchronization.
use tokio::time; // Importing time module from tokio for sleep functionality.

// Asynchronously fetches and displays market trades for a specific product.
pub async fn fetch_market_trades_handler(mut stop: watch::Receiver<()>) -> Result<()> {
    let product_id = prompt_for_product_id(); // Prompting user for product ID.
    let mut header_printed = false; // Flag to check if header has been printed.

    loop {
        // Using tokio::select! to handle concurrent tasks.
        tokio::select! {
            // Checking if the stop signal has been received.
            _ = stop.changed() => {
                println!("Stopping fetch_market_trades_handler"); // Print message when stopping.
                break; // Exit the loop.
            }
            // Adding a delay of 200 milliseconds before the next fetch.
            _ = time::sleep(Duration::from_millis(200)) => {
                // Fetching market trades and handling the result.
                match fetch_market_trades(&product_id).await {
                    Ok(market_trades) => display_market_trades(&market_trades, &mut header_printed), // Displaying market trades.
                    Err(e) => println!("Error fetching market trades for {}: {:?}", product_id, e), // Handling fetch error.
                }
            }
        }
    }
    Ok(()) // Returning Ok result.
}

// Function to display the market trades in a table format.
fn display_market_trades(trades: &Vec<MarketTrade>, header_printed: &mut bool) {
    let mut table = Table::new(); // Creating a new table.
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR); // Setting table format.

    // Printing the header if it hasn't been printed yet.
    if !*header_printed {
        table.set_titles(Row::new(vec![
            Cell::new("Trade ID"),
            Cell::new("Product ID"),
            Cell::new("Price"),
            Cell::new("Size"),
            Cell::new("Time"),
            Cell::new("Side"),
        ]));
        *header_printed = true; // Setting the header printed flag.
    }

    // Adding each trade to the table.
    for trade in trades {
        table.add_row(Row::new(vec![
            Cell::new(&format!("{:<15}", trade.trade_id)),
            Cell::new(&format!("{:<10}", trade.product_id)),
            Cell::new(&format!("{:<10}", trade.price)),
            Cell::new(&format!("{:<10}", trade.size)),
            Cell::new(&format!("{:<25}", trade.time)),
            Cell::new(&format!("{:<5}", trade.side)),
        ]));
    }

    table.printstd(); // Printing the table.
}
