mod html;
mod info;

use crate::html::Manager;
use crate::info::Info;
use denops_debugger_core::external::fetch::fetch;
use futures::channel::mpsc::{channel, Receiver, Sender, TryRecvError};
use futures::prelude::stream::{IntoStream, Next};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;
use v8_inspector_api_types::messages::Message;
use v8_inspector_api_types::prelude::{methods, Message as Msg, Method, WebSocketConnectionInfo};

#[derive(Debug)]
enum TestMsg {
    Msg(String),
    Terminate,
}

#[tokio::main]
async fn main() {
    let (mut stx, mut srx) = channel::<bool>(1);
    let (mut tx, mut rx) = channel::<TestMsg>(10);

    let info = Info::from_str("http://localhost:9229").unwrap();
    let man = Manager::new(info);
    let mut b = WebSocketManager::new(man.get_ws_cli().await.unwrap(), rx, srx);

    let main_thread = async move {
        let command = methods::Enable {};
        let data = command.into_method_call(1);
        let data = serde_json::to_string(data.as_ref()).unwrap();

        sleep(Duration::from_millis(1000)).await;
        tx.send(TestMsg::Msg(data)).await.unwrap();
        sleep(Duration::from_millis(5000)).await;

        stx.send(true).await.unwrap();
        tx.send(TestMsg::Terminate).await.unwrap();
        sleep(Duration::from_millis(1000)).await;
    };
    tokio::join!(b.reader, b.writer, main_thread);
    // tokio::select! {
    //     a1 = b.reader => {},
    //     b1 = b.writer => {},
    //     c1 = main_thread => {},
    // }
}

async fn get_stream(url: Url) -> anyhow::Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    Ok(connect_async(url).await?.0)
}

struct WebSocketManager {
    reader: JoinHandle<()>,
    writer: JoinHandle<()>,
}

impl WebSocketManager {
    pub fn new(
        stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
        mut rx: Receiver<TestMsg>,
        mut shutdown_rx: Receiver<bool>,
    ) -> Self {
        let (mut writer, mut reader) = stream.split();

        // create thread to manage sending message
        let writer = tokio::spawn(async move {
            'outer: while let Some(data) = rx.next().await {
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
                        break 'outer;
                    }
                }
            }
        });

        // create thread to manage reading message
        let reader = tokio::spawn(async move {
            // pending flush buffer and read message if possible.
            'outer: loop {
                if let Some(message) = reader.try_next().await.unwrap() {
                    if let Ok(res) = serde_json::from_str::<Msg>(message.to_text().unwrap()) {
                        match res {
                            Msg::Event(eve) => {
                                match tokio::io::stdout()
                                    .write_all(format!("recv[]: {:?}\n", eve).as_bytes())
                                    .await
                                {
                                    Ok(_) => {}
                                    Err(_) => {
                                        eprintln!("Error caused when reading stream");
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
                                    }
                                }
                            }
                            Msg::ConnectionShutdown => {
                                break 'outer;
                            }
                        }
                    }
                } else {
                    eprintln!("Error caused when reading stream");
                }
                if let Ok(msg) = shutdown_rx.try_next() {
                    if let Some(msg) = msg {
                        if msg {
                            break 'outer;
                        }
                    } else {
                        eprintln!("waiting ...");
                    }
                }
            }
            eprintln!("fin...");
        });

        WebSocketManager { reader, writer }
    }
}
