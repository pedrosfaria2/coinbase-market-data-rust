use anyhow::Result;
use tokio::sync::watch;
use tokio::{self, signal};
use crate::handlers::{
    market_trades_handler::fetch_market_trades_handler, 
    product_book_handler::fetch_product_book_handler,
    specific_product_handler::fetch_specific_product_handler,
};
use std::io::{self, Write}; // Import standard I/O library
use crate::clear_screen;

// Fetches and prints all data for a specific product
pub async fn fetch_all_data() -> Result<()> {
    let product_id = prompt_for_product_id(); // Prompt the user to enter a product ID
    let (tx, rx) = watch::channel(());

    // Spawn all handlers concurrently
    let product_book_handle = tokio::spawn(fetch_product_book_handler(rx.clone(), product_id.clone()));
    let market_trades_handle = tokio::spawn(fetch_market_trades_handler(rx.clone(), product_id.clone()));
    let specific_product_handle = tokio::spawn(fetch_specific_product_handler(rx.clone(), product_id.clone()));

    // Wait for Ctrl+C signal to stop all handlers
    signal::ctrl_c().await?;
    drop(tx); // Dropping the sender to signal the tasks to stop

    // Clear the screen after receiving the stop signal
    clear_screen::clear_screen();

    // Await all handler futures to complete
    product_book_handle.await??;
    market_trades_handle.await??;
    specific_product_handle.await??;

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
    let (tx, rx) = watch::channel(());
    let product_id = prompt_for_product_id();
    let handle = tokio::spawn(task(rx, product_id));
    let result = signal::ctrl_c().await;
    drop(tx); // Dropping the sender to signal the task to stop

    // Clear the screen after receiving the stop signal
    clear_screen::clear_screen();

    handle.await??;
    result?;
    Ok(())
}
