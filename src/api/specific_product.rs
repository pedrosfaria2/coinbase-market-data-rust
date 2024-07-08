use crate::models::Product;
use crate::utils::BASE_URL;
use anyhow::Result;
use reqwest::header::{HeaderMap, CONTENT_TYPE};

pub async fn fetch_specific_product(product_id: &str) -> Result<Product> {
    let url = format!("{}/market/products/{}", BASE_URL, product_id);

    let client = reqwest::Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse()?);

    let request = client.request(reqwest::Method::GET, &url).headers(headers);

    let response = request.send().await?;

    let product: Product = response.json().await?;
    Ok(product)
}
