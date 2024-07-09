use crate::api::product_book::fetch_product_book; // Importing the fetch_product_book function.
use crate::models::ProductBook; // Importing the ProductBook model.
use anyhow::Result; // Importing the Result type from anyhow for error handling.
use std::time::Duration; // Importing Duration for time handling.
use tokio::sync::watch; // Importing watch from tokio for synchronization.
use tokio::time; // Importing time module from tokio for sleep functionality;

// Asynchronously fetches and displays the product book for a specific product.
pub async fn fetch_product_book_handler(
    mut stop: watch::Receiver<()>,
    product_id: String,
) -> Result<()> {
    loop {
        // Using tokio::select! to handle concurrent tasks.
        tokio::select! {
            // Checking if the stop signal has been received.
            _ = stop.changed() => {
                println!("Stopping fetch_product_book_handler"); // Print message when stopping.
                break; // Exit the loop.
            }
            // Adding a delay of 50 milliseconds before the next fetch.
            _ = time::sleep(Duration::from_millis(750)) => {
                // Fetching the product book and handling the result.
                match fetch_product_book(&product_id).await {
                    Ok(product_book) => {
                        // Move cursor to the specific location for this handler
                        print!("\x1B[1;150H");
                        display_product_book(&product_book, 2); // Starting at line 2
                    }
                    Err(e) => println!("Error fetching product book for {}: {:?}", product_id, e),
                }
            }
        }
    }
    Ok(())
}

// Function to display the product book with levels of depth.
fn display_product_book(product_book: &ProductBook, start_y: usize) {
    let max_levels = 30
        .min(product_book.pricebook.bids.len())
        .min(product_book.pricebook.asks.len());

    // Print header
    println!(
        "{}{:<10} {:<10} {:<10} {:<10}",
        format!("\x1B[{};150H", start_y),
        "Price",
        "Bid Size",
        "Price",
        "Ask Size"
    );

    // Print each row
    for i in 0..max_levels {
        let bid = &product_book.pricebook.bids[i];
        let ask = &product_book.pricebook.asks[i];

        println!(
            "{}{:<10} {:<10} {:<10} {:<10}",
            format!("\x1B[{};150H", start_y + i + 1),
            bid.price,
            bid.size,
            ask.price,
            ask.size
        );
    }
}
