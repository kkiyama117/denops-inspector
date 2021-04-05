use crate::external::fetch::fetch;
use crate::external::ws_cli::WSStream;
use async_trait::async_trait;
use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use url::Url;
use v8_inspector_api_types::http_methods::{Version, WebSocketConnectionInfo};

// type WSCliSelectTask =
//     Pin<Box<dyn Future<Output = Option<WebSocketConnectionInfo>> + 'static + Send>>;

/// You can implement HTTP manager for your own because of this trait is used instead of concrete types.
#[async_trait]
pub trait HTTPManager {
    async fn get_worker_list(&self) -> Option<Vec<WebSocketConnectionInfo>>;
    async fn get_ws_cli<F>(&self, selector: F) -> Option<WSStream>
    where
        F: Fn(
                Vec<WebSocketConnectionInfo>,
            ) -> Pin<Box<dyn Future<Output = Option<WebSocketConnectionInfo>> + Send>>
            + Sync
            + Send;
}

#[derive(Debug)]
struct HTTPManagerError {}

impl Error for HTTPManagerError {}

impl fmt::Display for HTTPManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Manager Error")
    }
}

pub struct Manager {
    url: Url,
}

impl Manager {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub fn from_string(value: impl ToString) -> Option<Self> {
        let url = Url::parse(value.to_string().as_str()).ok()?;
        Some(Self { url })
    }

    pub async fn check_version(&self) -> Option<Version> {
        if let Ok(url) = &self.url.join("json/version") {
            crate::external::fetch::fetch(url.clone()).await.ok()
        } else {
            None
        }
    }
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
    fn get_ws_cli<'life0, 'async_trait, F>(
        &'life0 self,
        selector: F,
    ) -> Pin<Box<dyn Future<Output = Option<WSStream>> + Send + 'async_trait>>
    where
        F: Fn(
            Vec<WebSocketConnectionInfo>,
        ) -> Pin<Box<dyn Future<Output = Option<WebSocketConnectionInfo>> + Send>>,
        F: 'async_trait + Sync,
        'life0: 'async_trait,
        Self: 'async_trait,
        F: Send,
    {
        Box::pin(async move {
            let processes = self.get_worker_list().await;
            match processes {
                None => return None,
                Some(processes) => {
                    if let Some(p) = selector(processes).await {
                        log_info!("{:?}", p.clone());
                        let a = WSStream::get_stream(p.clone().web_socket_debugger_url)
                            .await
                            .ok();
                        a
                    } else {
                        None
                    }
                }
            }
        })
    }
}
