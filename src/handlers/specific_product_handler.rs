use crate::api::specific_product::fetch_specific_product; // Importing the fetch_specific_product function.
use crate::models::Product; // Importing the Product model.
use crate::prompt_for_product_id; // Importing the prompt_for_product_id function.
use anyhow::Result; // Importing the Result type from anyhow for error handling.
use prettytable::{format, Cell, Row, Table}; // Importing prettytable for table formatting.
use std::time::Duration; // Importing Duration from std for handling time intervals.
use tokio::sync::watch; // Importing watch from tokio for asynchronous synchronization.
use tokio::time; // Importing time from tokio for time handling.

// Asynchronously fetches and displays specific product information repeatedly until stopped.
pub async fn fetch_specific_product_handler(mut stop: watch::Receiver<()>) -> Result<()> {
    let product_id = prompt_for_product_id(); // Prompting the user for a product ID.
    let mut header_printed = false; // Flag to check if the header has been printed.

    loop {
        tokio::select! {
            _ = stop.changed() => { // Check if the stop signal has been received.
                println!("Stopping fetch_specific_product_handler");
                break;
            }
            _ = time::sleep(Duration::from_millis(200)) => { // Wait for 200 milliseconds before fetching data again.
                match fetch_specific_product(&product_id).await { // Fetch the specific product data.
                    Ok(product) => display_specific_product(&product, &mut header_printed), // Display the product data if successful.
                    Err(e) => println!("Error fetching product data for {}: {:?}", product_id, e), // Print error message if there's an error.
                }
            }
        }
    }
    Ok(())
}

// Displays the specific product information in a formatted table.
fn display_specific_product(product: &Product, header_printed: &mut bool) {
    let mut table = Table::new(); // Creating a new table.
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR); // Setting the table format.

    if !*header_printed {
        // Check if the header has not been printed.
        table.set_titles(Row::new(vec![
            Cell::new(&format!("{:<20}", "Product ID")),
            Cell::new(&format!("{:<15}", "Price")),
            Cell::new(&format!("{:<20}", "24h Change %")),
            Cell::new(&format!("{:<20}", "Volume 24h")),
            Cell::new(&format!("{:<20}", "Volume 24h Change %")),
            Cell::new(&format!("{:<10}", "Status")),
            Cell::new(&format!("{:<10}", "Base")),
            Cell::new(&format!("{:<10}", "Quote")),
        ]));
        *header_printed = true; // Set the flag to true to indicate the header has been printed.
    }

    // Adding a row with the specific product information.
    table.add_row(Row::new(vec![
        Cell::new(&format!("{:<20}", product.product_id)),
        Cell::new(&format!("{:<15}", product.price)),
        Cell::new(&format!("{:<20}", product.price_percentage_change_24h)),
        Cell::new(&format!("{:<20}", product.volume_24h)),
        Cell::new(&format!("{:<20}", product.volume_percentage_change_24h)),
        Cell::new(&format!("{:<10}", product.status)),
        Cell::new(&format!("{:<10}", product.base_name)),
        Cell::new(&format!("{:<10}", product.quote_name)),
    ]));

    table.printstd(); // Printing the table to the standard output.
}
