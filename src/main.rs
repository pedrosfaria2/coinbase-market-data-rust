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
use std::io::{self, Write};
use tokio::sync::watch;
use tokio::{self, signal};

#[tokio::main]
async fn main() -> Result<()> {
    loop {
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

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u8>().unwrap_or(0);

        match choice {
            1 => fetch_products_handler().await?,
            2 => fetch_server_time_handler().await?,
            3 => start_looping_task(fetch_product_book_handler).await?,
            4 => fetch_candles_handler().await?,
            5 => start_looping_task(fetch_market_trades_handler).await?,
            6 => start_looping_task(fetch_specific_product_handler).await?,
            7 => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
    Ok(())
}

fn prompt_for_product_id() -> String {
    print!("Enter the product ID: ");
    io::stdout().flush().unwrap();
    let mut product_id = String::new();
    io::stdin().read_line(&mut product_id).unwrap();
    product_id.trim().to_string()
}

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
