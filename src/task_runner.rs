use crate::handlers::{
    market_trades_handler::fetch_market_trades_handler,
    product_book_handler::fetch_product_book_handler,
    specific_product_handler::fetch_specific_product_handler,
};
use anyhow::Result;
use std::io::{self, Write}; // Import standard I/O library
use tokio::sync::watch;
use tokio::{self, signal};

// Fetches and prints all data for a specific product
pub async fn fetch_all_data(rx: watch::Receiver<()>, product_id: String) -> Result<()> {
    // Spawn all handlers concurrently
    let product_book_handle =
        tokio::spawn(fetch_product_book_handler(rx.clone(), product_id.clone()));
    let market_trades_handle =
        tokio::spawn(fetch_market_trades_handler(rx.clone(), product_id.clone()));
    let specific_product_handle = tokio::spawn(fetch_specific_product_handler(rx, product_id));

    // Wait for all handlers to complete and collect results
    let (product_book_res, market_trades_res, specific_product_res) = tokio::join!(
        product_book_handle,
        market_trades_handle,
        specific_product_handle
    );

    // Check for any errors
    if let Err(e) = product_book_res {
        eprintln!("Error fetching product book: {:?}", e);
    }
    if let Err(e) = market_trades_res {
        eprintln!("Error fetching market trades: {:?}", e);
    }
    if let Err(e) = specific_product_res {
        eprintln!("Error fetching specific product data: {:?}", e);
    }

    // Clear the screen after processing the data
    clear_screen();

    Ok(())
}

// Prompt the user to enter a product ID
pub fn prompt_for_product_id() -> String {
    print!("Enter the product ID: ");
    io::stdout().flush().unwrap();
    let mut product_id = String::new();
    io::stdin().read_line(&mut product_id).unwrap();
    product_id.trim().to_string()
}

// Start a looping task that listens for a stop signal (Ctrl+C)
pub async fn start_looping_task<F, Fut>(task: F) -> Result<()>
where
    F: Fn(watch::Receiver<()>, String) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<()>> + Send + 'static,
{
    loop {
        let (tx, rx) = watch::channel(());
        let product_id = prompt_for_product_id();
        let mut handle = tokio::spawn(task(rx.clone(), product_id.clone()));

        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("Received Ctrl+C, stopping...");
                drop(tx); // Dropping the sender to signal the task to stop

                // Wait for the task to complete
                if let Err(e) = handle.await {
                    eprintln!("Task failed: {:?}", e);
                }
                break; // Exit the loop after handling Ctrl+C
            },
            result = &mut handle => {
                if let Err(e) = result {
                    eprintln!("Task failed: {:?}", e);
                }
            }
        }

        // Clear the screen after receiving the stop signal
        clear_screen();
    }

    Ok(())
}

// Function to clear the screen
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
