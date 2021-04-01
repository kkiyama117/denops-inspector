use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

pub struct WSStream(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl WSStream {
    pub async fn get_stream(url: Url) -> anyhow::Result<Self> {
        Ok(WSStream(connect_async(url).await?.0))
    }
}
