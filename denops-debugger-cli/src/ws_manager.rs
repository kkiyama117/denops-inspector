use crate::types::WsStream;
use crate::{log_debug, log_error};
use futures::stream::SplitStream;
use futures_util::sink::Sink;
use futures_util::stream::Stream;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use std::error::Error;
use std::fmt;
use tokio::spawn;
use tokio::sync::watch::Receiver;
use tokio::task::JoinHandle;
use tungstenite::Message;
use v8_inspector_api_types::messages::Message as Msg;

#[derive(Clone, Debug)]
pub enum Command {
    Init,
    Msg(String),
    Terminate,
}

pub struct WebSocketManager {
    pub reader: JoinHandle<()>,
    pub writer: JoinHandle<()>,
}

#[derive(Debug)]
struct WebsocketManagerError {
    pub msg: String,
}
impl fmt::Display for WebsocketManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Manager Error")
    }
}
impl Error for WebsocketManagerError {}

impl WebSocketManager {
    pub fn new(stream: WsStream, rx: Receiver<Command>) -> Self {
        let (reader, writer) = create_ws_resolver(stream, rx);
        WebSocketManager { reader, writer }
    }
}

fn create_ws_resolver(
    stream: WsStream,
    rx: Receiver<Command>,
) -> (tokio::task::JoinHandle<()>, tokio::task::JoinHandle<()>) {
    let (writer, reader) = stream.into_stream().split();

    // create thread to manage sending message
    let rx1 = rx.clone();
    let writer = spawn(async move {
        if writer_process(writer, rx1).await.is_ok() {
            log_debug!("writer finished successfully!")
        }
    });

    let rx2 = rx;
    // create thread to manage reading message
    let reader = spawn(async move {
        if reader_process(reader, rx2).await.is_ok() {
            log_debug!("reader finished successfully!")
        }
    });
    (reader, writer)
}

async fn reader_process<S: Stream<Item = Result<T, E>> + Unpin, T: ToString, E: Error>(
    mut reader: SplitStream<S>,
    shutdown_rx: Receiver<Command>,
) -> Result<(), WebsocketManagerError> {
    // pending flush buffer and read message if possible.
    'outer: loop {
        match reader.try_next().await {
            Ok(message) => {
                if let Some(message) = message {
                    let message = message.to_string();
                    if !message.is_empty() {
                        match serde_json::from_str::<Msg>(message.as_str()) {
                            Ok(res) => match res {
                                Msg::Event(eve) => {
                                    log_debug!("recv[]: {:?}", eve);
                                }
                                Msg::Response(res) => {
                                    log_debug!("recv[]: {:?}", res);
                                }
                                Msg::ConnectionShutdown => {
                                    break 'outer;
                                }
                            },
                            Err(e) => {
                                log_error!(
                                    "Error caused when parsing data in responses ({})",
                                    message
                                );
                                log_error!("{:?}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                return Err(WebsocketManagerError {
                    msg: format!("Error caused when reading stream({})", e),
                });
            }
        }
        // check received shutdown message or not.
        if let Command::Terminate = *shutdown_rx.borrow() {
            break 'outer;
        }
    }
    Ok(())
}

async fn writer_process(
    mut writer: impl Sink<Message> + Unpin,
    mut rx: Receiver<Command>,
) -> Result<(), WebsocketManagerError> {
    while rx.changed().await.is_ok() {
        let data = rx.borrow().clone();
        // write message
        match data {
            Command::Msg(data) => {
                if writer.send(data.into()).await.is_err() {
                    return Err(WebsocketManagerError {
                        msg: "Error caused when writing stream".to_string(),
                    });
                }
            }
            Command::Terminate => {
                break;
            }
            _ => {}
        }
    }
    if writer.close().await.is_err() {
        return Err(WebsocketManagerError {
            msg: "Error occurred in closing websocket!".to_string(),
        });
    }
    Ok(())
}
