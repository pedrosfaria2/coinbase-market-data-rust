use anyhow::Result;
use crate::handlers::{
    candles_handler::fetch_candles_handler, market_trades_handler::fetch_market_trades_handler,
    product_book_handler::fetch_product_book_handler, products_handler::fetch_products_handler,
    server_time_handler::fetch_server_time_handler,
    specific_product_handler::fetch_specific_product_handler,
};
use crate::task_runner::{start_looping_task, fetch_all_data};
use crate::clear_screen;
use std::io::{self, Write};

pub async fn show_menu() -> Result<()> {
    loop {
 // Clear the screen before displaying the menu
        // Display menu options to the user
        println!("");
        println!("Menu:");
        println!("1. Fetch and print all products");
        println!("2. Fetch and print server time");
        println!("3. Fetch and print order book for a specific product");
        println!("4. Fetch and print candles for a specific product");
        println!("5. Fetch and print market trades for a specific product");
        println!("6. Fetch and print data for a specific product");
        println!("7. Fetch and print all data (trades, specific product, book) for a specific product");
        println!("8. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        // Read and parse user input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u8>().unwrap_or(0);

        // Match the user's choice and execute the corresponding handler
        match choice {
            1 => {
                clear_screen::clear_screen();
                fetch_products_handler().await?;
            }
            2 => {
                clear_screen::clear_screen();
                fetch_server_time_handler().await?;
            }
            3 => {
                clear_screen::clear_screen();
                start_looping_task(fetch_product_book_handler).await?;
            }
            4 => {
                clear_screen::clear_screen();
                fetch_candles_handler().await?;
            }
            5 => {
                clear_screen::clear_screen();
                start_looping_task(fetch_market_trades_handler).await?;
            }
            6 => {
                clear_screen::clear_screen();
                start_looping_task(fetch_specific_product_handler).await?;
            }
            7 => {
                clear_screen::clear_screen();
                fetch_all_data().await?;
            }
            8 => break, // Exit the loop
            _ => println!("Invalid choice, please try again."),
        }
    }
    Ok(())
}
