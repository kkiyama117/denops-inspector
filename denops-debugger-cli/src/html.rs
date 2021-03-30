use crate::get_stream;
use crate::info::Info;
use denops_debugger_core::external::fetch::fetch;
use std::error::Error;
use std::fmt;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use v8_inspector_api_types::http_methods::WebSocketConnectionInfo;

pub struct Manager {
    info: Info,
}

impl Manager {
    pub fn new(info: Info) -> Self {
        Self { info }
    }
}

#[derive(Debug)]
struct ManagerError {}
impl fmt::Display for ManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Manager Error")
    }
    /* ... */
}
impl Error for ManagerError {}

impl Manager {
    pub async fn get_process_list(&self) -> Result<Vec<WebSocketConnectionInfo>, ManagerError> {
        let url = self.info.base_url.join("json").unwrap();
        match fetch::<Vec<WebSocketConnectionInfo>>(url).await {
            Ok(v) => Ok(v),
            Err(_) => Err(ManagerError {}),
        }
    }
    pub async fn get_ws_cli(&self) -> anyhow::Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let processes = self.get_process_list().await?;
        let process = processes.get(0);
        match process {
            None => Err((ManagerError {}).into()),
            Some(p) => Ok(get_stream(p.web_socket_debugger_url.clone()).await?),
        }
    }
}
