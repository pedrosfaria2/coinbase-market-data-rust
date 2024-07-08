use anyhow::Result;
use crate::models::{ProductsResponse, Product};
use crate::utils::BASE_URL;

pub async fn fetch_products() -> Result<Vec<Product>> {
    let url = format!("{}/market/products", BASE_URL);
    let response = reqwest::get(&url).await?;

    // Attempt to deserialize the JSON response
    let products_response: ProductsResponse = response.json().await?;
    Ok(products_response.products)
}
