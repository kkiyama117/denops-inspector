use reqwest::{Error, Response};
use serde::de::DeserializeOwned;

pub async fn fetch<T: DeserializeOwned>(url: &str) -> Result<T, Box<dyn std::error::Error>> {
    return Ok(reqwest::get(url).await?.json::<T>().await?);
}
