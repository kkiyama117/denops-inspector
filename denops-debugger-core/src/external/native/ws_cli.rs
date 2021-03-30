use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

pub(crate) type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub async fn get_stream(url: Url) -> anyhow::Result<WSStream> {
    Ok(connect_async(url).await?.0)
}
