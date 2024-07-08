use crate::api::market_trades::fetch_market_trades;
use crate::models::MarketTrade;
use crate::prompt_for_product_id;
use anyhow::Result;
use prettytable::{format, Cell, Row, Table};
use std::time::Duration;
use tokio::sync::watch;
use tokio::time;

pub async fn fetch_market_trades_handler(mut stop: watch::Receiver<()>) -> Result<()> {
    let product_id = prompt_for_product_id();
    let mut header_printed = false;
    loop {
        tokio::select! {
            _ = stop.changed() => {
                println!("Stopping fetch_market_trades_handler");
                break;
            }
            _ = time::sleep(Duration::from_millis(10)) => {
                match fetch_market_trades(&product_id).await {
                    Ok(market_trades) => display_market_trades(&market_trades, &mut header_printed),
                    Err(e) => println!("Error fetching market trades for {}: {:?}", product_id, e),
                }
            }
        }
    }
    Ok(())
}

fn display_market_trades(trades: &Vec<MarketTrade>, header_printed: &mut bool) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    if !*header_printed {
        table.set_titles(Row::new(vec![
            Cell::new("Trade ID"),
            Cell::new("Product ID"),
            Cell::new("Price"),
            Cell::new("Size"),
            Cell::new("Time"),
            Cell::new("Side"),
        ]));
        *header_printed = true;
    }

    for trade in trades {
        table.add_row(Row::new(vec![
            Cell::new(&format!("{:<15}", trade.trade_id)),
            Cell::new(&format!("{:<10}", trade.product_id)),
            Cell::new(&format!("{:<10}", trade.price)),
            Cell::new(&format!("{:<10}", trade.size)),
            Cell::new(&format!("{:<25}", trade.time)),
            Cell::new(&format!("{:<5}", trade.side)),
        ]));
    }

    table.printstd();
}
