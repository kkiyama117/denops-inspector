use serde::de::DeserializeOwned;
use url::Url;

pub async fn fetch<T: DeserializeOwned>(url: Url) -> Result<T, anyhow::Error> {
    return Ok(reqwest::get(url).await?.json::<T>().await?);
}
