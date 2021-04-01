use crate::external::ws_cli::WSStream;
use crate::external::{spawn, JoinHandle};
use futures::channel::mpsc::Receiver;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use v8_inspector_api_types::messages::Message;

#[derive(Debug)]
pub enum TestMsg {
    Msg(String),
    Terminate,
}

pub struct WebSocketManager {
    pub reader: JoinHandle<()>,
    pub writer: JoinHandle<()>,
}

impl WebSocketManager {
    pub fn new(
        stream: WSStream,
        mut rx: Receiver<TestMsg>,
        mut shutdown_rx: Receiver<bool>,
    ) -> Self {
        let (mut writer, mut reader) = stream.split();

        // create thread to manage sending message
        let writer = spawn(async move {
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
                        match writer.close().await {
                            Ok(_) => {}
                            Err(_) => {
                                log_error!("Error occurred in closing websocket!")
                            }
                        }
                        break 'outer;
                    }
                }
            }
        });

        // create thread to manage reading message
        let reader = spawn(async move {
            // pending flush buffer and read message if possible.
            'outer: loop {
                if let Some(message) = reader.try_next().await.unwrap() {
                    if let Ok(res) = serde_json::from_str::<Message>(message.to_text().unwrap()) {
                        match res {
                            Message::Event(eve) => {
                                log_info!("recv[]: {:?}", eve);
                            }
                            Message::Response(res) => {
                                log_info!("recv[]: {:?}", res);
                            }
                            Message::ConnectionShutdown => {
                                break 'outer;
                            }
                        }
                    }
                } else {
                    log_error!("Error caused when reading stream");
                }
                // check received shutdown message or not.
                if let Ok(msg) = shutdown_rx.try_next() {
                    if let Some(msg) = msg {
                        if msg {
                            break 'outer;
                        }
                    } else {
                        log_error!("waiting ...");
                    }
                }
            }
            log_info!("fin...");
        });

        WebSocketManager { reader, writer }
    }
}
