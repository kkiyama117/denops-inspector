use async_trait::async_trait;
use denops_debugger_core::client::HTTPManager;
use denops_debugger_core::external::ws_cli::WSStream;
use futures::Future;
use serde::de::DeserializeOwned;
use std::pin::Pin;
use url::Url;
use v8_inspector_api_types::http_methods::{Version, WebSocketConnectionInfo};

pub struct Manager {
    url: Url,
}

impl Manager {
    // pub fn new(url: Url) -> Self {
    //     Self { url }
    // }

    pub fn from_string(value: impl ToString) -> Option<Self> {
        let url = Url::parse(value.to_string().as_str()).ok()?;
        Some(Self { url })
    }

    // pub async fn check_version(&self) -> Option<Version> {
    //     if let Ok(url) = &self.url.join("json/version") {
    //         return fetch(url.clone()).await.ok();
    //     }
    //     None
    // }
}
#[async_trait]
impl HTTPManager for Manager {
    async fn get_worker_list(&self) -> Option<Vec<WebSocketConnectionInfo>> {
        if let Ok(url) = self.url.join("json") {
            match fetch::<Vec<WebSocketConnectionInfo>>(url).await {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    /// get websocket client from info given by asynchronous closure.
    async fn get_ws_cli<F>(&self, selector: F) -> Option<WSStream>
    where
        F: Fn(
                Vec<WebSocketConnectionInfo>,
            ) -> Pin<Box<dyn Future<Output = Option<WebSocketConnectionInfo>> + Send>>
            + Sync
            + Send,
    {
        if let Some(processes) = self.get_worker_list().await {
            println!("{:?}", processes);
            return if let Some(p) = selector(processes).await {
                let a = WSStream::get_stream(p.clone().web_socket_debugger_url)
                    .await
                    .ok();
                a
            } else {
                None
            };
        }
        None
    }
}

async fn fetch<T: DeserializeOwned>(url: Url) -> Result<T, anyhow::Error> {
    return Ok(reqwest::get(url).await?.json::<T>().await?);
}
