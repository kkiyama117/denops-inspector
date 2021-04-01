use crate::external::fetch::fetch;
use crate::external::ws_cli::{get_stream, WSStream};
use async_trait::async_trait;
use futures::Future;
use std::error::Error;
use std::fmt;
use std::pin::Pin;
use url::Url;
use v8_inspector_api_types::http_methods::{Version, WebSocketConnectionInfo};

#[async_trait]
pub trait HTTPManager {
    async fn get_worker_list(&self) -> Option<Vec<WebSocketConnectionInfo>>;
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
}

#[derive(Debug)]
struct ManagerError {}

impl fmt::Display for ManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Manager Error")
    }
}

impl Error for ManagerError {}

impl Manager {
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
}
type WSCliSelectTask =
    Pin<Box<dyn Future<Output = Option<WebSocketConnectionInfo>> + 'static + Send>>;

pub async fn get_ws_cli<F>(manager: impl HTTPManager, selector: F) -> Option<WSStream>
where
    // F: Fn(Vec<WebSocketConnectionInfo>) -> Option<WebSocketConnectionInfo>,
    F: FnOnce(Vec<WebSocketConnectionInfo>) -> WSCliSelectTask,
{
    let processes = manager.get_worker_list().await?;
    if let Some(p) = selector(processes).await {
        log_info!("{:?}", p.clone());
        get_stream(p.clone().web_socket_debugger_url).await.ok()
    } else {
        None
    }
    // match process {
    //     None => Err((ManagerError {}).into()),
    //     Some(p) => Ok(get_stream(p.web_socket_debugger_url.clone()).await?),
    // }
}
