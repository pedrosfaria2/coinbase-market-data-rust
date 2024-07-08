use crate::api::candles::fetch_candles;
use crate::models::Candle;
use anyhow::Result;
use prettytable::{format, Cell, Row, Table};
use std::io::{self, Write};

pub async fn fetch_candles_handler() -> Result<()> {
    let product_id = prompt_for_product_id();
    let start = prompt_for_start_time();
    let end = prompt_for_end_time();
    let granularity = prompt_for_granularity();
    let mut header_printed = false;

    // Calculate the number of candles to be requested
    let start_dt = chrono::DateTime::parse_from_rfc3339(&start)?.with_timezone(&chrono::Utc);
    let end_dt = chrono::DateTime::parse_from_rfc3339(&end)?.with_timezone(&chrono::Utc);
    let duration = end_dt.signed_duration_since(start_dt);
    let num_candles = duration.num_seconds() / granularity_to_seconds(&granularity);

    if num_candles > 300 {
        println!("The number of candles requested should be less than 300. Please adjust the time range or granularity.");
        return Ok(());
    }

    match fetch_candles(&product_id, &start, &end, &granularity).await {
        Ok(candles) => {
            if candles.is_empty() {
                println!("No candle data available for the given time range and granularity.");
            } else {
                display_candles(&candles, &mut header_printed);
            }
        }
        Err(e) => println!("Error fetching candles for {}: {:?}", product_id, e),
    }

    Ok(())
}

fn display_candles(candles: &Vec<Candle>, header_printed: &mut bool) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    if !*header_printed {
        table.set_titles(Row::new(vec![
            Cell::new("Start Time"),
            Cell::new("Low"),
            Cell::new("High"),
            Cell::new("Open"),
            Cell::new("Close"),
            Cell::new("Volume"),
        ]));
        *header_printed = true;
    }

    for candle in candles {
        table.add_row(Row::new(vec![
            Cell::new(&format!("{:<20}", candle.start)),
            Cell::new(&format!("{:<10}", candle.low)),
            Cell::new(&format!("{:<10}", candle.high)),
            Cell::new(&format!("{:<10}", candle.open)),
            Cell::new(&format!("{:<10}", candle.close)),
            Cell::new(&format!("{:<10}", candle.volume)),
        ]));
    }

    table.printstd();
}

fn prompt_for_product_id() -> String {
    print!("Enter the product ID: ");
    io::stdout().flush().unwrap();
    let mut product_id = String::new();
    io::stdin().read_line(&mut product_id).unwrap();
    product_id.trim().to_string()
}

fn prompt_for_start_time() -> String {
    print!("Enter the start time (e.g., 2022-01-01T00:00:00Z): ");
    io::stdout().flush().unwrap();
    let mut start_time = String::new();
    io::stdin().read_line(&mut start_time).unwrap();
    start_time.trim().to_string()
}

fn prompt_for_end_time() -> String {
    print!("Enter the end time (e.g., 2022-01-02T00:00:00Z): ");
    io::stdout().flush().unwrap();
    let mut end_time = String::new();
    io::stdin().read_line(&mut end_time).unwrap();
    end_time.trim().to_string()
}

fn prompt_for_granularity() -> String {
    println!("Enter the granularity (e.g., ONE_MINUTE, FIVE_MINUTE, FIFTEEN_MINUTE, THIRTY_MINUTE, ONE_HOUR, TWO_HOUR, SIX_HOUR, ONE_DAY): ");
    io::stdout().flush().unwrap();
    let mut granularity = String::new();
    io::stdin().read_line(&mut granularity).unwrap();
    granularity.trim().to_string()
}

fn granularity_to_seconds(granularity: &str) -> i64 {
    match granularity {
        "ONE_MINUTE" => 60,
        "FIVE_MINUTE" => 300,
        "FIFTEEN_MINUTE" => 900,
        "THIRTY_MINUTE" => 1800,
        "ONE_HOUR" => 3600,
        "TWO_HOUR" => 7200,
        "SIX_HOUR" => 21600,
        "ONE_DAY" => 86400,
        _ => 0,
    }
}
