use crate::api::product_book::fetch_product_book; // Importing the fetch_product_book function.
use crate::models::ProductBook; // Importing the ProductBook model.
use crate::prompt_for_product_id; // Importing the prompt_for_product_id function.
use anyhow::Result; // Importing the Result type from anyhow for error handling.
use std::time::Duration; // Importing Duration from std::time para o sleep.
use tokio::sync::watch; // Importing watch from tokio para sincronização.
use tokio::time; // Importing time module from tokio para funcionalidades de sleep;

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
            // Adding a delay of 10 milliseconds before the next fetch.
            _ = time::sleep(Duration::from_millis(50)) => {
                // Fetching the product book and handling the result.
                match fetch_product_book(&product_id).await {
                    Ok(product_book) => {
                        print!("\x1B[2J\x1B[1;1H");
                        display_product_book(&product_book);
                    }
                    Err(e) => println!("Error fetching product book for {}: {:?}", product_id, e),
                }
            }
        }
    }
    Ok(())
}

// Function to display the product book with levels of depth.
fn display_product_book(product_book: &ProductBook) {
    let max_levels = 20
        .min(product_book.pricebook.bids.len())
        .min(product_book.pricebook.asks.len());

    println!(
        "{:<10} {:<10} {:<10} {:<10}",
        "Price", "Bid Size", "Price", "Ask Size"
    );

    for i in 0..max_levels {
        let bid = &product_book.pricebook.bids[i];
        let ask = &product_book.pricebook.asks[i];

        println!(
            "{:<10} {:<10} {:<10} {:<10}",
            bid.price, bid.size, ask.price, ask.size
        );
    }
    println!();
}
