use crate::external::ws_cli::WSStream;
use crate::external::{spawn, JoinHandle};
use futures::channel::mpsc::Receiver;
use futures::stream::SplitStream;
use futures_util::sink::Sink;
use futures_util::stream::Stream;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use std::error::Error;
use std::fmt;
use v8_inspector_api_types::messages::{Event, Message as Msg};

#[derive(Debug)]
pub enum TestMsg {
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
    pub fn new(stream: WSStream, rx: Receiver<TestMsg>, shutdown_rx: Receiver<bool>) -> Self {
        let (reader, writer) = create_ws_resolver(stream, rx, shutdown_rx).unwrap();
        WebSocketManager { reader, writer }
    }
}

fn create_ws_resolver(
    stream: WSStream,
    rx: Receiver<TestMsg>,
    shutdown_rx: Receiver<bool>,
) -> Result<(JoinHandle<()>, JoinHandle<()>), WebsocketManagerError> {
    let (writer, reader) = stream.inner().split();

    // create thread to manage sending message
    let writer = spawn(async move {
        if writer_process(writer, rx).await.is_ok() {
            log_debug!("writer finished successfully!")
        }
    });

    // create thread to manage reading message
    let reader = spawn(async move {
        if reader_process(reader, shutdown_rx).await.is_ok() {
            log_debug!("reader finished successfully!")
        }
    });
    Ok((reader, writer))
}

async fn reader_process<S: Stream<Item = Result<T, E>> + Unpin, T: ToString, E>(
    mut reader: SplitStream<S>,
    mut shutdown_rx: Receiver<bool>,
) -> Result<(), WebsocketManagerError> {
    // pending flush buffer and read message if possible.
    'outer: loop {
        if let Ok(message) = reader.try_next().await {
            if let Some(message) = message {
                let message = message.to_string();
                if let Ok(res) = serde_json::from_str::<Msg>(message.as_str()) {
                    match res {
                        Msg::Event(eve) => match eve {
                            Event::ScriptParsed(_) => {}
                            _ => {
                                log_debug!("recv[]: {:?}", eve);
                            }
                        },
                        Msg::Response(res) => {
                            log_debug!("recv[]: {:?}", res);
                        }
                        Msg::ConnectionShutdown => {
                            break 'outer;
                        }
                    }
                } else {
                    log_error!("Error caused when parsing data in responses");
                    log_error!("{:?}", message);
                }
            } else {
                return Err(WebsocketManagerError {
                    msg: "Error caused when reading stream".to_string(),
                });
            }
        }
        // check received shutdown message or not.
        if let Ok(msg) = shutdown_rx.try_next() {
            if let Some(msg) = msg {
                if msg {
                    break 'outer;
                }
            } else {
                log_debug!("waiting ...");
            }
        }
    }
    Ok(())
}

async fn writer_process(
    mut writer: impl Sink<crate::external::ws_cli::Message> + Unpin,
    mut rx: Receiver<TestMsg>,
) -> Result<(), WebsocketManagerError> {
    while let Some(data) = rx.next().await {
        // write message
        match data {
            TestMsg::Msg(data) => match writer.send(data.into()).await {
                Ok(_) => {}
                Err(_) => {
                    return Err(WebsocketManagerError {
                        msg: "Error caused when writing stream".to_string(),
                    });
                }
            },
            TestMsg::Terminate => {
                rx.close();
                match writer.close().await {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(WebsocketManagerError {
                            msg: "Error occurred in closing websocket!".to_string(),
                        })
                    }
                }
                break;
            }
        }
    }
    return Ok(());
}
