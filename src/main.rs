mod api;
mod handlers;
mod models;
mod utils;

use anyhow::Result;
use handlers::{
    candles_handler::fetch_candles_handler, market_trades_handler::fetch_market_trades_handler,
    product_book_handler::fetch_product_book_handler, products_handler::fetch_products_handler,
    server_time_handler::fetch_server_time_handler,
    specific_product_handler::fetch_specific_product_handler,
};
use std::io::{self, Write}; // Import standard I/O library
use tokio::sync::watch; // Import watch for task signaling
use tokio::{self, signal}; // Import tokio and signal handling

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        // Display menu options to the user
        println!("Menu:");
        println!("1. Fetch and print all products");
        println!("2. Fetch and print server time");
        println!("3. Fetch and print order book for a specific product");
        println!("4. Fetch and print candles for a specific product");
        println!("5. Fetch and print market trades for a specific product");
        println!("6. Fetch and print data for a specific product");
        println!("7. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        // Read and parse user input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u8>().unwrap_or(0);

        // Match the user's choice and execute the corresponding handler
        match choice {
            1 => fetch_products_handler().await?,    // Fetch all products
            2 => fetch_server_time_handler().await?, // Fetch server time
            3 => start_looping_task(fetch_product_book_handler).await?, // Fetch order book
            4 => fetch_candles_handler().await?,     // Fetch candles
            5 => start_looping_task(fetch_market_trades_handler).await?, // Fetch market trades
            6 => start_looping_task(fetch_specific_product_handler).await?, // Fetch specific product data
            7 => break,                                                     // Exit the loop
            _ => println!("Invalid choice, please try again."),
        }
    }
    Ok(())
}

// Prompt the user to enter a product ID
fn prompt_for_product_id() -> String {
    print!("Enter the product ID: ");
    io::stdout().flush().unwrap();
    let mut product_id = String::new();
    io::stdin().read_line(&mut product_id).unwrap();
    product_id.trim().to_string()
}

// Start a looping task that listens for a stop signal (Ctrl+C)
async fn start_looping_task<F, Fut>(task: F) -> Result<()>
where
    F: Fn(watch::Receiver<()>) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<()>> + Send + 'static,
{
    let (tx, rx) = watch::channel(());
    let handle = tokio::spawn(task(rx));
    signal::ctrl_c().await?;
    drop(tx); // Dropping the sender to signal the task to stop
    handle.await??;
    Ok(())
}
