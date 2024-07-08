use reqwest::Error;
use crate::models::ProductBook;
use crate::utils::BASE_URL;

pub async fn fetch_product_book(product_id: &str) -> Result<ProductBook, Error> {
    let url = format!("{}/market/product_book?product_id={}", BASE_URL, product_id);
    let response = reqwest::get(&url).await?.json::<ProductBook>().await?;
    Ok(response)
}
