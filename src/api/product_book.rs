use crate::models::ProductBook; // Importing the ProductBook model.
use crate::utils::BASE_URL; // Importing the base URL for API requests.
use reqwest::Error; // Importing the Error type from reqwest for handling request errors.

// Asynchronously fetches the product book for a specific product.
pub async fn fetch_product_book(product_id: &str) -> Result<ProductBook, Error> {
    // Returns a Result with a ProductBook object or a reqwest::Error.

    // Constructing the API request URL.
    let url = format!("{}/market/product_book?product_id={}", BASE_URL, product_id);

    // Sending the GET request to the API and attempting to deserialize the JSON response into a ProductBook object.
    let response = reqwest::get(&url).await?.json::<ProductBook>().await?;

    // Returning the deserialized ProductBook object.
    Ok(response)
}
