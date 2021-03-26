use denops_debugger_core::external::fetch::fetch;
use futures_util::{future, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;
use v8_inspector_api_types::commands;
use v8_inspector_api_types::prelude::*;

#[tokio::main]
async fn main() {
    // let mut dc = DebuggerClient::new("http://localhost:9229/".parse().unwrap());
    let info = Info::from_str("http://localhost:9229").unwrap();
    let man = Manager::new(info);
    println!("{:?}", &man.get_process_list().await.unwrap());
    let b = WebSocketManager::new(man.get_ws_cli().await.unwrap());
    b.open().await;

    // dbg!(dc.check_version().await);

    // let a = commands::Enable {};
    // let data = a.into_method_call(1);
    // let dc = dc.open().await;
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize)]
struct Info {
    pub base_url: Url,
}

impl Info {
    pub fn new(base_url: Url) -> Self {
        Self { base_url }
    }
}

impl FromStr for Info {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Url::parse(s) {
            Ok(base_url) => Ok(Self::new(base_url)),
            Err(e) => Err(e),
        }
    }
}

struct Manager {
    info: Info,
}

impl Manager {
    fn new(info: Info) -> Self {
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

async fn get_stream(url: Url) -> anyhow::Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    Ok(connect_async(url).await?.0)
}

struct WebSocketManager {
    stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
}

impl WebSocketManager {
    pub fn new(stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>) -> Self {
        WebSocketManager { stream }
    }

    pub async fn open(self) {
        let a = commands::Enable {};
        let data = a.into_method_call(1);
        let (mut writer, mut reader) = self.stream.split();

        // create thread to manage sending message
        let write_thread = tokio::spawn(async move {
            // send message and wait data
            // write message
            match writer
                .send(serde_json::to_string(data.as_ref()).unwrap().into())
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Error caused when writing stream")
                }
            }
        });

        // create thread to manage reading message
        let read_thread = tokio::spawn(async move {
            // pending flush buffer and read message if possible.
            let message = reader.next().await.unwrap().unwrap();
            match tokio::io::stdout()
                .write_all(format!("recv[]: {}\n", message).as_bytes())
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Error caused when reading stream")
                }
            }
        });
        // run all threads.
        let res = future::try_join(write_thread, read_thread).await;
        match res {
            Ok((_, _)) => {
                println!("successfully finished");
            }
            Err(e) => {
                eprintln!("Error in thread=>\n{}", e);
            }
        }
    }
}
