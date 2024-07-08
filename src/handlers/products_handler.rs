use anyhow::Result;
use crate::api::products::fetch_products;
use prettytable::{Table, Row, Cell, format, row};
use crate::models::Product;
use std::io::{self, Write};

pub async fn fetch_products_handler() -> Result<()> {
    let products = match fetch_products().await {
        Ok(products) => products,
        Err(e) => {
            println!("Error fetching products: {:?}", e);
            return Ok(());
        }
    };

    loop {
        println!("Choose display mode:");
        println!("1. Synthetic view");
        println!("2. Complete view");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u8>().unwrap_or(0);

        match choice {
            1 => display_synthetic_view(&products),
            2 => display_complete_view(&products),
            _ => {
                println!("Invalid choice, please try again.");
                continue;
            }
        }
        break;
    }
    Ok(())
}

fn display_synthetic_view(products: &Vec<Product>) {
    let mut table = Table::new();
    table.add_row(row!["Product ID", "Price", "24h Change %", "Volume 24h", "Base Name", "Quote Name", "Status"]);

    for product in products {
        table.add_row(row![
            product.product_id,
            product.price,
            product.price_percentage_change_24h,
            product.volume_24h,
            product.base_name,
            product.quote_name,
            product.status
        ]);
    }

    table.printstd();
}

fn display_complete_view(products: &Vec<Product>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.add_row(Row::new(vec![
        Cell::new("Product ID"),
        Cell::new("Price"),
        Cell::new("24h Change %"),
        Cell::new("Volume 24h"),
        Cell::new("Volume 24h Change %"),
        Cell::new("Base Increment"),
        Cell::new("Quote Increment"),
        Cell::new("Quote Min Size"),
        Cell::new("Quote Max Size"),
        Cell::new("Base Min Size"),
        Cell::new("Base Max Size"),
        Cell::new("Base Name"),
        Cell::new("Quote Name"),
        Cell::new("Watched"),
        Cell::new("Is Disabled"),
        Cell::new("New"),
        Cell::new("Status"),
        Cell::new("Cancel Only"),
        Cell::new("Limit Only"),
        Cell::new("Post Only"),
        Cell::new("Trading Disabled"),
        Cell::new("Auction Mode"),
        Cell::new("Product Type"),
        Cell::new("Quote Currency ID"),
        Cell::new("Base Currency ID"),
        Cell::new("FCM Trading Session Details"),
        Cell::new("Mid Market Price"),
        Cell::new("Alias"),
        Cell::new("Alias To"),
        Cell::new("Base Display Symbol"),
        Cell::new("Quote Display Symbol"),
        Cell::new("View Only"),
        Cell::new("Price Increment"),
        Cell::new("Display Name"),
        Cell::new("Product Venue"),
        Cell::new("Approximate Quote 24h Volume"),
    ]));

    for product in products {
        table.add_row(Row::new(vec![
            Cell::new(&product.product_id),
            Cell::new(&product.price),
            Cell::new(&product.price_percentage_change_24h),
            Cell::new(&product.volume_24h),
            Cell::new(&product.volume_percentage_change_24h),
            Cell::new(&product.base_increment),
            Cell::new(&product.quote_increment),
            Cell::new(&product.quote_min_size),
            Cell::new(&product.quote_max_size),
            Cell::new(&product.base_min_size),
            Cell::new(&product.base_max_size),
            Cell::new(&product.base_name),
            Cell::new(&product.quote_name),
            Cell::new(&format_bool(product.watched)),
            Cell::new(&format_bool(product.is_disabled)),
            Cell::new(&format_bool(product.new)),
            Cell::new(&product.status),
            Cell::new(&format_bool(product.cancel_only)),
            Cell::new(&format_bool(product.limit_only)),
            Cell::new(&format_bool(product.post_only)),
            Cell::new(&format_bool(product.trading_disabled)),
            Cell::new(&format_bool(product.auction_mode)),
            Cell::new(&product.product_type),
            Cell::new(&product.quote_currency_id),
            Cell::new(&product.base_currency_id),
            Cell::new(&format!("{:?}", product.fcm_trading_session_details)),
            Cell::new(&product.mid_market_price),
            Cell::new(&product.alias),
            Cell::new(&format!("{:?}", product.alias_to)),
            Cell::new(&product.base_display_symbol),
            Cell::new(&product.quote_display_symbol),
            Cell::new(&format_bool(product.view_only)),
            Cell::new(&product.price_increment),
            Cell::new(&product.display_name),
            Cell::new(&product.product_venue),
            Cell::new(&product.approximate_quote_24h_volume),
        ]));
    }

    table.printstd();
}

fn format_bool(value: bool) -> String {
    if value {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}
