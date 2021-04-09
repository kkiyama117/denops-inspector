use crate::types::WsStream;
use async_trait::async_trait;
use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use v8_inspector_api_types::http_methods::WebSocketConnectionInfo;

// type WSCliSelectTask =
//     Pin<Box<dyn Future<Output = Option<WebSocketConnectionInfo>> + 'static + Send>>;

/// You can implement HTTP manager for your own because of this trait is used instead of concrete types.
#[async_trait]
pub trait HttpManager {
    async fn get_worker_list(&self) -> Option<Vec<WebSocketConnectionInfo>>;
    async fn get_ws_cli<F>(&self, selector: F) -> Option<WsStream>
    where
        F: Fn(
                Vec<WebSocketConnectionInfo>,
            ) -> Pin<Box<dyn Future<Output = Option<WebSocketConnectionInfo>> + Send>>
            + Sync
            + Send;
}

#[derive(Debug)]
struct HttpManagerError {}

impl Error for HttpManagerError {}

impl fmt::Display for HttpManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Manager Error")
    }
}
