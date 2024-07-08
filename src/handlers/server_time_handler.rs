use crate::api::server_time::fetch_server_time;
use anyhow::Result;

pub async fn fetch_server_time_handler() -> Result<()> {
    match fetch_server_time().await {
        Ok(server_time) => println!("Server Time: {:?}", server_time),
        Err(e) => println!("Error fetching server time: {:?}", e),
    }
    Ok(())
}
