use crate::models::ServerTime; // Importing the ServerTime model.
use crate::utils::BASE_URL; // Importing the base URL for API requests.
use anyhow::Result; // Importing the Result type from anyhow for error handling.

// Asynchronously fetches the current server time.
pub async fn fetch_server_time() -> Result<ServerTime> {
    // Returns a Result with a ServerTime object or an error.

    // Constructing the API request URL for fetching the server time.
    let url = format!("{}/time", BASE_URL);

    // Sending the GET request to the API.
    let response = reqwest::get(&url).await?;

    // Attempting to deserialize the JSON response into a ServerTime object.
    let server_time: ServerTime = response.json().await?;

    // Returning the deserialized ServerTime object.
    Ok(server_time)
}
