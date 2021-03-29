use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;
use v8_inspector_api_types::prelude::*;

struct WebSocket {
    stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
}
impl WebSocket {
    pub fn new(stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>) -> Self {
        WebSocket { stream }
    }
}

pub struct DebuggerClient {
    url: Url,
    ws: Option<WebSocket>,
}

impl DebuggerClient {
    pub fn new(url: Url) -> Self {
        DebuggerClient { url, ws: None }
    }

    fn get_base_url(&self) -> Url {
        self.url.clone()
    }

    pub async fn check_version(&self) -> Version {
        let url = self.get_base_url();
        let url = url.join("json/version").unwrap();
        crate::external::fetch::fetch(url).await.unwrap()
    }

    pub async fn open(&mut self) -> Self {
        let base_list = self.get_worker_list().await;
        let base = base_list.get(0).unwrap();
        let (ws_stream, _) = connect_async(base.web_socket_debugger_url.clone())
            .await
            .expect("Failed to connect");

        Self {
            url: self.url.clone(),
            ws: Some(WebSocket::new(ws_stream)),
        }
    }

    async fn get_worker_list(&self) -> Vec<WebSocketConnectionInfo> {
        let url = self.get_base_url();
        let url = url.join("json").unwrap();
        crate::external::fetch::fetch::<Vec<WebSocketConnectionInfo>>(url)
            .await
            .unwrap()
    }

    // async fn send_method<T:Method>(&self, method: Box<T>) -> Vec<Response> {
    //     let result = vec![];
    //     let stream = self.ws.unwrap().
    // }
}
