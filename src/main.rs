mod api;
mod clear_screen;
mod handlers;
mod menu;
mod models;
mod task_runner;
mod utils;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    menu::show_menu().await
}
