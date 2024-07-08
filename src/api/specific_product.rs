use crate::models::Product; // Importing the Product model.
use crate::utils::BASE_URL; // Importing the base URL for API requests.
use anyhow::Result; // Importing the Result type from anyhow for error handling.
use reqwest::header::{HeaderMap, CONTENT_TYPE}; // Importing HeaderMap and CONTENT_TYPE for setting request headers.

// Asynchronously fetches the details of a specific product by its ID.
pub async fn fetch_specific_product(product_id: &str) -> Result<Product> {
    // Returns a Result with a Product object or an error.

    // Constructing the API request URL for fetching a specific product by ID.
    let url = format!("{}/market/products/{}", BASE_URL, product_id);

    // Creating a new reqwest client.
    let client = reqwest::Client::builder().build()?; // Handling possible errors with the ? operator.

    // Creating and setting the headers for the request.
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse()?); // Setting the Content-Type header to application/json.

    // Building the GET request with the specified URL and headers.
    let request = client.request(reqwest::Method::GET, &url).headers(headers);

    // Sending the request and awaiting the response.
    let response = request.send().await?;

    // Attempting to deserialize the JSON response into a Product object.
    let product: Product = response.json().await?;

    // Returning the deserialized Product object.
    Ok(product)
}
