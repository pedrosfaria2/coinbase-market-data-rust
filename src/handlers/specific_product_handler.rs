use crate::api::specific_product::fetch_specific_product;
use crate::models::Product;
use crate::prompt_for_product_id;
use anyhow::Result;
use prettytable::{format, Cell, Row, Table};
use std::time::Duration;
use tokio::sync::watch;
use tokio::time;

pub async fn fetch_specific_product_handler(mut stop: watch::Receiver<()>) -> Result<()> {
    let product_id = prompt_for_product_id();
    let mut header_printed = false;
    loop {
        tokio::select! {
            _ = stop.changed() => {
                println!("Stopping fetch_specific_product_handler");
                break;
            }
            _ = time::sleep(Duration::from_millis(200)) => {
                match fetch_specific_product(&product_id).await {
                    Ok(product) => display_specific_product(&product, &mut header_printed),
                    Err(e) => println!("Error fetching product data for {}: {:?}", product_id, e),
                }
            }
        }
    }
    Ok(())
}

fn display_specific_product(product: &Product, header_printed: &mut bool) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    if !*header_printed {
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
        *header_printed = true;
    }

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

    table.printstd();
}
