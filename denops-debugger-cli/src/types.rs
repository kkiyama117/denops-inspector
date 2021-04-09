use std::fmt::{Debug, Display, Formatter, Result};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub struct WsStream(WebSocketStream<MaybeTlsStream<TcpStream>>);
impl WsStream {
    pub fn new(inner: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self(inner)
    }
    pub fn into_stream(self) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        self.0
    }
}

impl Debug for WsStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "WsStream for tokio runtime")
    }
}
impl Display for WsStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "WsStream for tokio runtime")
    }
}

pub type CallId = u32;
