use crate::api::server_time::fetch_server_time; // Importing the fetch_server_time function.
use anyhow::Result; // Importing the Result type from anyhow for error handling.

// Asynchronously fetches the server time and displays it.
pub async fn fetch_server_time_handler() -> Result<()> {
    // Fetching the server time and handling potential errors.
    match fetch_server_time().await {
        Ok(server_time) => println!("Server Time: {:?}", server_time), // If successful, print the server time.
        Err(e) => println!("Error fetching server time: {:?}", e), // Print error message if there's an error.
    }
    Ok(())
}
