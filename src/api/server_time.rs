use anyhow::Result;
use crate::models::ServerTime;
use crate::utils::BASE_URL;

pub async fn fetch_server_time() -> Result<ServerTime> {
    let url = format!("{}/time", BASE_URL);
    let response = reqwest::get(&url).await?;

    // Attempt to deserialize the JSON response
    let server_time: ServerTime = response.json().await?;
    Ok(server_time)
}
