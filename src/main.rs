mod api;
mod handlers;
mod models;
mod utils;
mod menu;
mod clear_screen;
mod task_runner;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    menu::show_menu().await
}
