use denops_debugger_core::external::fetch::fetch;
use futures::prelude::stream::SplitStream;
use futures_util::{future, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::error::Error;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;
use v8_inspector_api_types::prelude::methods;
use v8_inspector_api_types::{
    methods::{Method, MethodCall},
    parse_response,
    prelude::WebSocketConnectionInfo,
    Message as Msg,
};

#[derive(Debug)]
enum TestMsg {
    Msg(String),
    Terminate,
}

#[tokio::main]
async fn main() {
    let info = Info::from_str("http://localhost:9229").unwrap();
    let man = Manager::new(info);
    let b = WebSocketManager::new(man.get_ws_cli().await.unwrap());

    let a = methods::Enable {};
    let data = a.into_method_call(1);
    let data = serde_json::to_string(data.as_ref()).unwrap();
    b.tx.send(TestMsg::Msg(data)).await.unwrap();

    let a = methods::Disable {};
    let data = a.into_method_call(2);
    let data = serde_json::to_string(data.as_ref()).unwrap();
    b.tx.send(TestMsg::Msg(data)).await.unwrap();
    // b.tx.send(TestMsg::Terminate).await.unwrap();

    b.writer.await.unwrap();
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
    tx: Sender<TestMsg>,
    reader: JoinHandle<()>,
    writer: JoinHandle<()>,
}

impl WebSocketManager {
    pub fn new(stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        let (mut writer, mut reader) = stream.split();
        let (mut tx, mut rx) = mpsc::channel::<TestMsg>(10);

        // create thread to manage sending message
        let writer = tokio::spawn(async move {
            while let Some(data) = rx.recv().await {
                // write message
                match data {
                    TestMsg::Msg(data) => match writer.send(data.into()).await {
                        Ok(_) => {}
                        Err(_) => {
                            eprintln!("Error caused when writing stream");
                        }
                    },
                    TestMsg::Terminate => {
                        rx.close();
                        writer.close().await;
                    }
                }
            }
        });

        // create thread to manage reading message
        let reader = tokio::spawn(async move {
            // pending flush buffer and read message if possible.
            while let Ok(message) = reader.next().await.unwrap() {
                // let res = message;
                // match tokio::io::stdout()
                //     .write_all(format!("{}\n", res).as_bytes())
                //     .await
                // {
                //     Ok(_) => {}
                //     Err(_) => {
                //         eprintln!("Error caused when reading stream");
                //         break;
                //     }
                // }
                let res = serde_json::from_str::<Msg>(message.to_text().unwrap()).unwrap();
                match res {
                    Msg::Event(eve) => {
                        match tokio::io::stdout()
                            .write_all(format!("recv[]: {:?}\n", eve).as_bytes())
                            .await
                        {
                            Ok(_) => {}
                            Err(_) => {
                                eprintln!("Error caused when reading stream");
                                break;
                            }
                        }
                    }
                    Msg::Response(res) => {
                        match tokio::io::stdout()
                            .write_all(format!("recv[]: {:?}\n", res).as_bytes())
                            .await
                        {
                            Ok(_) => {}
                            Err(_) => {
                                eprintln!("Error caused when reading stream");
                                break;
                            }
                        }
                    }
                    Msg::ConnectionShutdown => {
                        break;
                    }
                }
            }
        });

        WebSocketManager { tx, reader, writer }
    }

    pub async fn shutdown(self) {
        self.writer.await.unwrap();
        self.reader.abort();
    }
}
