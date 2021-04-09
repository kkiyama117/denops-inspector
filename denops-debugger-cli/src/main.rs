use futures::future::BoxFuture;
use futures::join;
use futures_util::FutureExt;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio::sync::watch::{channel, Sender};
use tokio::time::{sleep, Duration};

use v8_inspector_api_types::http_methods::WebSocketConnectionInfo;
use v8_inspector_api_types::{methods::Method, protocols::debugger};

use crate::client::HttpManager;
use crate::manager::Manager;
use crate::ws_manager::{Command, WebSocketManager};
use serde::Serialize;
use tokio::sync::watch::error::SendError;
use tokio::task::JoinHandle;
use v8_inspector_api_types::protocols::runtime;

pub mod client;
pub(crate) mod manager;
pub(crate) mod ws_manager;

#[macro_use]
pub mod logging;

mod transport;
pub(crate) mod types;

#[tokio::main]
async fn main() {
    let InitializedValue { tx, ws_manager: b }: InitializedValue = initialize().await;

    let io_main = main_thread(tx);

    let _ = join!(io_main, b.reader, b.writer);
}
async fn main_thread(tx: Sender<Command>) -> JoinHandle<()> {
    let _main_thread = tokio::spawn(async move {
        let mut input_lines = BufReader::new(stdin());
        let mut count = 0;
        // Init Debug session.
        let command = debugger::methods::Enable {
            max_script_cache_size: None,
        };
        if let Err(e) = send_command(&tx, command, count).await {
            log_error!("{:?}", e);
        }
        count += 1;
        sleep(Duration::from_millis(1000)).await;

        let command = runtime::methods::RunIfWaitingForDebugger {};
        if let Err(e) = send_command(&tx, command, count).await {
            log_error!("{:?}", e);
        }
        count += 1;
        loop {
            let mut buf = String::new();
            match input_lines.read_line(&mut buf).await {
                Ok(remain) => {
                    if remain == 0 {
                        log_info!("EOF!");
                    }
                    if !buf.is_empty() {
                        if buf == "terminate\n" {
                            break;
                        } else if buf == "pause\n" {
                            let command = debugger::methods::Pause {};
                            if let Err(e) = send_command(&tx, command, count).await {
                                log_error!("{:?}", e);
                            }
                            // for next loop
                            count += 1;
                            sleep(Duration::from_millis(100)).await;
                        } else if buf == "resume\n" {
                            let command = debugger::methods::Resume {
                                terminate_on_resume: false,
                            };
                            if let Err(e) = send_command(&tx, command, count).await {
                                log_error!("{:?}", e);
                            }
                            // for next loop
                            count += 1;
                            sleep(Duration::from_millis(100)).await;
                        } else {
                            let command = debugger::methods::RestartFrame {
                                call_frame_id: buf.trim().to_string(),
                            };
                            if let Err(e) = send_command(&tx, command, count).await {
                                log_error!("{:?}", e);
                            }

                            // for next loop
                            count += 1;
                            sleep(Duration::from_millis(100)).await;
                        }
                    } else {
                        log_info!("skip!");
                    }
                }
                Err(e) => {
                    log_error!("{:?}", e);
                }
            }
        }

        // send messages to terminate other threads
        if tx.send(Command::Terminate).is_err() {}
        if tx.send(Command::Terminate).is_err() {}
        sleep(Duration::from_millis(100)).await;
    });
    return _main_thread;
}

async fn send_command<T>(
    tx: &Sender<Command>,
    method: impl Method<ReturnObject = T> + Serialize + Sized,
    count: u32,
) -> Result<(), SendError<Command>> {
    let method_call = &method.into_method_call(count);
    let data = serde_json::to_string(method_call).unwrap();
    return tx.send(Command::Msg(data));
}

async fn initialize() -> InitializedValue {
    let (tx, rx) = channel::<Command>(Command::Init);

    let man = Manager::from_string("http://localhost:9229").unwrap();
    let stream = man.get_ws_cli(selector).await.unwrap();
    let ws_manager = WebSocketManager::new(stream, rx);
    InitializedValue { tx, ws_manager }
}

fn selector(
    x: Vec<WebSocketConnectionInfo>,
) -> BoxFuture<'static, Option<WebSocketConnectionInfo>> {
    async move { x.get(1).cloned() }.boxed()
}

struct InitializedValue {
    tx: Sender<Command>,
    ws_manager: WebSocketManager,
}
