use crate::api::product_book::fetch_product_book;
use crate::prompt_for_product_id;
use anyhow::Result;
use std::time::Duration;
use tokio::sync::watch;
use tokio::time;

pub async fn fetch_product_book_handler(mut stop: watch::Receiver<()>) -> Result<()> {
    let product_id = prompt_for_product_id();
    loop {
        tokio::select! {
            _ = stop.changed() => {
                println!("Stopping fetch_product_book_handler");
                break;
            }
            _ = time::sleep(Duration::from_secs(1)) => {
                match fetch_product_book(&product_id).await {
                    Ok(product_book) => println!("Product Book for {}: {:?}", product_id, product_book),
                    Err(e) => println!("Error fetching product book for {}: {:?}", product_id, e),
                }
            }
        }
    }
    Ok(())
}
