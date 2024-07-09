
# Cryptocurrency Market Data Query System

This project is a CLI system in Rust for querying and displaying cryptocurrency market data from the Coinbase API. It allows the user to choose various operations to query different types of market data, such as available products, server time, order book, candles, and market trades for specific products.

## Features

- **Query Products:** List all available cryptocurrency products on the platform.
- **Query Server Time:** Retrieve the current server time of the platform.
- **Query Order Book:** Display the order book (bids and asks) for a specific cryptocurrency product continuously.
- **Query Candles:** Retrieve OHLC (Open, High, Low, Close) candle data for a specific cryptocurrency product within a defined time range.
- **Query Market Trades:** Display market trades for a specific cryptocurrency product continuously.
- **Query Specific Product Data:** Display detailed information of a specific cryptocurrency product continuously.
- **Fetch All Data:** Simultaneously fetch and display the order book, market trades, and specific product data for a specific cryptocurrency product. This feature allows you to see a comprehensive view of the market data in real-time, updating all the selected data streams concurrently.

## Requirements

To run the project, you need to have Rust and Cargo installed. Additionally, the project depends on some external libraries managed by Cargo.

## Installation and Execution

1. **Clone the repository:**
   ```bash
   git clone https://github.com/pedrosfaria2/coinbase-market-data-rust.git
   cd coinbase-market-data-rust
   ```

2. **Build and run:**
   ```bash
   cargo build --release
   ./target/release/coinbase_api_client.exe
   ```

3. **Usage:**
    - Upon starting the program, you will be presented with a menu of numbered options.
    - Choose the desired operation by typing the corresponding number and pressing Enter.
    - Follow the on-screen instructions to enter any additional information needed, such as the product ID, time range, etc.

4. **Stop execution:**
    - Press `Ctrl+C` to exit the program at any time.

## Detailed Feature Descriptions

### Fetch All Data

The Fetch All Data option (option 7 in the menu) is designed to provide a comprehensive view of the market data for a specific product. When selected, this feature will:

1. Prompt you to enter the product ID.
2. Start three concurrent tasks to fetch the order book, market trades, and specific product data for the given product ID.
3. Continuously display the fetched data in a synchronized manner, updating the information in real-time.
4. Allow you to stop the execution gracefully by pressing Ctrl+C, ensuring all running tasks are properly terminated.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
