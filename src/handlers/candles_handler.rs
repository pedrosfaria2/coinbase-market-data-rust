use crate::api::candles::fetch_candles; // Importing the fetch_candles function.
use crate::models::Candle; // Importing the Candle model.
use anyhow::Result; // Importing the Result type from anyhow for error handling.
use prettytable::{format, Cell, Row, Table}; // Importing prettytable components for table formatting and display.
use std::io::{self, Write}; // Importing I/O utilities for user input.

// Asynchronously fetches and displays candle data for a specific product.
pub async fn fetch_candles_handler() -> Result<()> {
    let product_id = prompt_for_product_id(); // Prompting user for product ID.
    let start = prompt_for_start_time(); // Prompting user for start time.
    let end = prompt_for_end_time(); // Prompting user for end time.
    let granularity = prompt_for_granularity(); // Prompting user for granularity.
    let mut header_printed = false; // Flag to check if header has been printed.

    // Converting start and end time to DateTime<Utc> and calculating the duration.
    let start_dt = chrono::DateTime::parse_from_rfc3339(&start)?.with_timezone(&chrono::Utc);
    let end_dt = chrono::DateTime::parse_from_rfc3339(&end)?.with_timezone(&chrono::Utc);
    let duration = end_dt.signed_duration_since(start_dt);
    let num_candles = duration.num_seconds() / granularity_to_seconds(&granularity); // Calculating number of candles.

    // Checking if the number of candles exceeds the limit.
    if num_candles > 300 {
        println!("The number of candles requested should be less than 300. Please adjust the time range or granularity.");
        return Ok(());
    }

    // Fetching candles and handling the result.
    match fetch_candles(&product_id, &start, &end, &granularity).await {
        Ok(candles) => {
            if candles.is_empty() {
                println!("No candle data available for the given time range and granularity.");
            // No data available.
            } else {
                display_candles(&candles, &mut header_printed); // Displaying candle data.
            }
        }
        Err(e) => println!("Error fetching candles for {}: {:?}", product_id, e), // Handling fetch error.
    }

    Ok(()) // Returning Ok result.
}

// Function to display the candles in a table format.
fn display_candles(candles: &Vec<Candle>, header_printed: &mut bool) {
    let mut table = Table::new(); // Creating a new table.
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR); // Setting table format.

    // Printing the header if it hasn't been printed yet.
    if !*header_printed {
        table.set_titles(Row::new(vec![
            Cell::new("Start Time"),
            Cell::new("Low"),
            Cell::new("High"),
            Cell::new("Open"),
            Cell::new("Close"),
            Cell::new("Volume"),
        ]));
        *header_printed = true; // Setting the header printed flag.
    }

    // Adding each candle to the table.
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

    table.printstd(); // Printing the table.
}

// Function to prompt the user for product ID.
fn prompt_for_product_id() -> String {
    print!("Enter the product ID: "); // Prompt message.
    io::stdout().flush().unwrap(); // Flushing stdout to display prompt.
    let mut product_id = String::new(); // Creating a mutable string for input.
    io::stdin().read_line(&mut product_id).unwrap(); // Reading user input.
    product_id.trim().to_string() // Returning the trimmed input.
}

// Function to prompt the user for start time.
fn prompt_for_start_time() -> String {
    print!("Enter the start time (e.g., 2022-01-01T00:00:00Z): "); // Prompt message.
    io::stdout().flush().unwrap(); // Flushing stdout to display prompt.
    let mut start_time = String::new(); // Creating a mutable string for input.
    io::stdin().read_line(&mut start_time).unwrap(); // Reading user input.
    start_time.trim().to_string() // Returning the trimmed input.
}

// Function to prompt the user for end time.
fn prompt_for_end_time() -> String {
    print!("Enter the end time (e.g., 2022-01-02T00:00:00Z): "); // Prompt message.
    io::stdout().flush().unwrap(); // Flushing stdout to display prompt.
    let mut end_time = String::new(); // Creating a mutable string for input.
    io::stdin().read_line(&mut end_time).unwrap(); // Reading user input.
    end_time.trim().to_string() // Returning the trimmed input.
}

// Function to prompt the user for granularity.
fn prompt_for_granularity() -> String {
    println!("Enter the granularity (e.g., ONE_MINUTE, FIVE_MINUTE, FIFTEEN_MINUTE, THIRTY_MINUTE, ONE_HOUR, TWO_HOUR, SIX_HOUR, ONE_DAY): "); // Prompt message.
    io::stdout().flush().unwrap(); // Flushing stdout to display prompt.
    let mut granularity = String::new(); // Creating a mutable string for input.
    io::stdin().read_line(&mut granularity).unwrap(); // Reading user input.
    granularity.trim().to_string() // Returning the trimmed input.
}

// Function to convert granularity to seconds.
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
        _ => 0, // Default case for invalid granularity.
    }
}
