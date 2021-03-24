use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;
use crate::external::ws::WebSocket;
use async_trait::async_trait;
use v8_inspector_api_types::prelude::*;


pub struct DebuggerClient<S> {
    url: Url,
    stream: WebSocketStream<MaybeTlsStream<S>>,
}

impl<S: tokio::stream::TcpStream> DebuggerClient<S> {
    pub async fn new(url: Url) -> Self {
        let (ws_stream, _) = connect_async(url.clone()).await.expect("Failed to connect");
        DebuggerClient {
            url,
            stream: ws_stream,
        }
    }

    fn get_base_url(&self) -> Url {
        self.url.clone()
    }
}


#[async_trait]
pub trait DebuggerClientTrait {
    async fn check_version(&self) -> Version;
    fn open(&self);
    async fn get_worker_list(&self) -> Vec<WebSocketConnectionInfo>;
    // async fn send_method<T>(&self, method: Box<dyn Method<ReturnObject = T>>) -> Vec<Response>;
}

#[async_trait]
impl DebuggerClientTrait for DebuggerClient<S> {
    async fn check_version(&self) -> Version {
        let mut url = self.get_base_url();
        url.push_str("json/version");
        crate::external::fetch::fetch(url).await.unwrap()
    }

    fn open(&self) {
        unimplemented!()
    }

    async fn get_worker_list(&self) -> Vec<WebSocketConnectionInfo> {
        let mut url = self.get_base_url();
        url.push_str("json");
        crate::external::fetch::fetch::<Vec<WebSocketConnectionInfo>>(url)
            .await
            .unwrap()
    }

    // async fn send_method<T>(&self, method: Box<dyn Method<ReturnObject = T>>) -> Vec<Response> {
    //     unimplemented!()
    // }
}
