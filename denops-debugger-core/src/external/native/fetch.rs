use serde::de::DeserializeOwned;
use url::Url;

pub(crate) async fn fetch<T: DeserializeOwned>(url: Url) -> Result<T, anyhow::Error> {
    return Ok(reqwest::get(url).await?.json::<T>().await?);
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use crate::external::fetch::fetch;
    use url::Url;
    use v8_inspector_api_types::prelude::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_fetch_http() {
        // run_client("ws://127.0.0.1:9229/ws/8f27e285-769d-4a7d-aef0-55f506bd7fc6").await;
        pretty_assertions::assert_eq!(
            fetch::<Version>(Url::parse("http://localhost:9229/json/version").unwrap())
                .await
                .unwrap()
                .v8_version,
            String::from(
                fetch::<serde_json::Value>(
                    Url::parse("http://localhost:9229/json/version").unwrap()
                )
                .await
                .unwrap()
                .get("V8-Version")
                .unwrap()
                .as_str()
                .unwrap()
            )
        );
        pretty_assertions::assert_eq!(
            fetch::<Vec<WebSocketConnectionInfo>>(
                Url::parse("http://localhost:9229/json").unwrap()
            )
            .await
            .unwrap()[0]
                .description,
            "deno"
        );
    }
}
