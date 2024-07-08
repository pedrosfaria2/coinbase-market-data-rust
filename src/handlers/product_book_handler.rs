use crate::api::product_book::fetch_product_book; // Importing the fetch_product_book function.
use crate::models::ProductBook; // Importing the ProductBook model.
use crate::prompt_for_product_id; // Importing the prompt_for_product_id function.
use anyhow::Result; // Importing the Result type from anyhow for error handling.
use prettytable::{format, Cell, Row, Table}; // Importing prettytable components for table formatting and display.
use std::collections::HashMap; // Importing HashMap for aggregating bid and ask sizes.
use std::time::Duration; // Importing Duration for time handling.
use tokio::sync::watch; // Importing watch from tokio for synchronization.
use tokio::time; // Importing time module from tokio for sleep functionality.

// Asynchronously fetches and displays the product book for a specific product.
pub async fn fetch_product_book_handler(mut stop: watch::Receiver<()>) -> Result<()> {
    let product_id = prompt_for_product_id(); // Prompting user for product ID.

    loop {
        // Using tokio::select! to handle concurrent tasks.
        tokio::select! {
            // Checking if the stop signal has been received.
            _ = stop.changed() => {
                println!("Stopping fetch_product_book_handler"); // Print message when stopping.
                break; // Exit the loop.
            }
            // Adding a delay of 200 milliseconds before the next fetch.
            _ = time::sleep(Duration::from_millis(200)) => {
                // Fetching the product book and handling the result.
                match fetch_product_book(&product_id).await {
                    Ok(product_book) => display_product_book(&product_book), // Displaying the product book.
                    Err(e) => println!("Error fetching product book for {}: {:?}", product_id, e), // Handling fetch error.
                }
            }
        }
    }
    Ok(()) // Returning Ok result.
}

// Function to display the product book with levels of depth.
fn display_product_book(product_book: &ProductBook) {
    let mut bids_map: HashMap<String, f64> = HashMap::new(); // Creating a HashMap to aggregate bid sizes by price level.
    let mut asks_map: HashMap<String, f64> = HashMap::new(); // Creating a HashMap to aggregate ask sizes by price level.

    // Aggregate bids by price level.
    for entry in &product_book.pricebook.bids {
        let size = entry.size.parse::<f64>().unwrap_or(0.0); // Parsing size to f64.
        *bids_map.entry(entry.price.clone()).or_insert(0.0) += size; // Aggregating sizes.
    }

    // Aggregate asks by price level.
    for entry in &product_book.pricebook.asks {
        let size = entry.size.parse::<f64>().unwrap_or(0.0); // Parsing size to f64.
        *asks_map.entry(entry.price.clone()).or_insert(0.0) += size; // Aggregating sizes.
    }

    // Convert the HashMaps to sorted Vecs.
    let mut bids: Vec<_> = bids_map.iter().collect(); // Collecting bids into a Vec.
    let mut asks: Vec<_> = asks_map.iter().collect(); // Collecting asks into a Vec.
    bids.sort_by(|a, b| b.0.partial_cmp(a.0).unwrap()); // Sorting bids in descending order.
    asks.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap()); // Sorting asks in ascending order.

    // Limit to top 15 to 20 levels of depth.
    let max_levels = 20; // Maximum levels to display.
    bids.truncate(max_levels); // Truncating bids to max_levels.
    asks.truncate(max_levels); // Truncating asks to max_levels.

    // Display the product book with levels of depth.
    let mut table = Table::new(); // Creating a new table.
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR); // Setting table format.

    // Print the header for each update.
    table.set_titles(Row::new(vec![
        Cell::new("Price"),
        Cell::new("Bid Size"),
        Cell::new("Price"),
        Cell::new("Ask Size"),
    ]));

    let empty_string = String::new(); // Placeholder for empty price.
    let empty_size = 0.0; // Placeholder for empty size.

    // Iterating through the levels of depth and adding rows to the table.
    for i in 0..max_levels {
        let (bid_price, bid_size) = if i < bids.len() {
            (bids[i].0, bids[i].1) // Getting bid price and size.
        } else {
            (&empty_string, &empty_size) // Using placeholders if no more bids.
        };
        let (ask_price, ask_size) = if i < asks.len() {
            (asks[i].0, asks[i].1) // Getting ask price and size.
        } else {
            (&empty_string, &empty_size) // Using placeholders if no more asks.
        };

        table.add_row(Row::new(vec![
            Cell::new(&format!("{:<10}", bid_price)),
            Cell::new(&format!("{:<10}", bid_size)),
            Cell::new(&format!("{:<10}", ask_price)),
            Cell::new(&format!("{:<10}", ask_size)),
        ]));
    }

    table.printstd(); // Printing the table.
    println!(); // Adding a newline for better readability.
    println!(); // Adding another newline for better readability.
}
