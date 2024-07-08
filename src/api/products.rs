use crate::models::{Product, ProductsResponse}; // Importing the Product and ProductsResponse models.
use crate::utils::BASE_URL; // Importing the base URL for API requests.
use anyhow::Result; // Importing the Result type from anyhow for error handling.

// Asynchronously fetches a list of all products.
pub async fn fetch_products() -> Result<Vec<Product>> {
    // Returns a Result with a vector of Product objects or an error.

    // Constructing the API request URL.
    let url = format!("{}/market/products", BASE_URL);

    // Sending the GET request to the API.
    let response = reqwest::get(&url).await?;

    // Attempting to deserialize the JSON response into a ProductsResponse object.
    let products_response: ProductsResponse = response.json().await?;

    // Returning the deserialized vector of Product objects.
    Ok(products_response.products)
}
