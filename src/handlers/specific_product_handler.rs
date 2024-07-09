use crate::api::specific_product::fetch_specific_product; // Importing the fetch_specific_product function.
use crate::models::Product; // Importing the Product model.
use anyhow::Result; // Importing the Result type from anyhow for error handling.
use std::time::Duration; // Importing Duration from std for handling time intervals.
use tokio::sync::watch; // Importing watch from tokio for asynchronous synchronization.
use tokio::time; // Importing time from tokio for time handling;

// Asynchronously fetches and displays specific product information repeatedly until stopped.
pub async fn fetch_specific_product_handler(
    mut stop: watch::Receiver<()>,
    product_id: String,
) -> Result<()> {
    loop {
        tokio::select! {
            _ = stop.changed() => { // Check if the stop signal has been received.
                println!("Stopping fetch_specific_product_handler");
                break;
            }
            _ = time::sleep(Duration::from_millis(300)) => { // Wait for 50 milliseconds before fetching data again.
                match fetch_specific_product(&product_id).await { // Fetch the specific product data.
                    Ok(product) => {
                        // Move cursor to the specific location for this handler
                        print!("\x1B[45;1H");
                        display_specific_product(&product); // Display the product data if successful.
                    },
                    Err(e) => println!("Error fetching product data for {}: {:?}", product_id, e), // Print error message if there's an error.
                }
            }
        }
    }
    Ok(())
}

// Displays the specific product information in a formatted table.
fn display_specific_product(product: &Product) {
    println!(
        "{:<20} {:<15} {:<20} {:<20} {:<20} {:<10} {:<10} {:<10}",
        "Product ID",
        "Price",
        "24h Change %",
        "Volume 24h",
        "Volume 24h Change %",
        "Status",
        "Base",
        "Quote"
    );

    println!(
        "{:<20} {:<15} {:<20} {:<20} {:<20} {:<10} {:<10} {:<10}",
        product.product_id,
        product.price,
        product.price_percentage_change_24h,
        product.volume_24h,
        product.volume_percentage_change_24h,
        product.status,
        product.base_name,
        product.quote_name
    );
}
