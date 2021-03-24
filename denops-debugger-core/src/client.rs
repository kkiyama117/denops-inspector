// use crate::external::ws::WebSocket;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

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
