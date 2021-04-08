use futures::future::BoxFuture;
use futures::join;
use futures_util::FutureExt;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio::time::{sleep, Duration};

use client::HTTPManager;
use tokio::sync::watch::{channel, Sender};
use v8_inspector_api_types::http_methods::WebSocketConnectionInfo;
use v8_inspector_api_types::{methods::Method, protocols::debugger::methods};

use crate::external::JoinHandle;
use crate::manager::Manager;
use crate::ws_manager::{Command, WebSocketManager};

pub mod client;
pub(crate) mod manager;
pub(crate) mod ws_manager;

#[path = "native/mod.rs"]
#[macro_use]
pub(crate) mod external;

#[tokio::main]
async fn main() {
    let InitializedValue {
        mut tx,
        ws_manager: b,
    }: InitializedValue = initialize().await;

    let io_main = main_thread(tx);

    let _ = join!(io_main, b.reader, b.writer);
}
async fn main_thread(tx: Sender<Command>) -> JoinHandle<()> {
    let _main_thread = tokio::spawn(async move {
        let mut input_lines = BufReader::new(stdin());
        let mut count = 0;
        loop {
            let mut buf = String::new();
            match input_lines.read_line(&mut buf).await {
                Ok(remain) => {
                    if remain == 0 {
                        println!("EOF!");
                    }
                    if buf == "terminate\n" {
                        break;
                    } else {
                        let command = methods::Enable {
                            max_script_cache_size: None,
                        };
                        println!("{}", serde_json::to_string(&command).unwrap());
                        // let cmd = serde_json::from_str::<MethodInput>(buf.as_str()).unwrap();
                        let data = command.into_method_call(count);
                        let data = serde_json::to_string(data.as_ref()).unwrap();
                        tx.send(Command::Msg(data));
                        // for next loop
                        count += 1;
                        sleep(Duration::from_millis(100)).await;
                        println!("next");
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

        // send messages to terminate other threads
        tx.send(Command::Terminate);
        tx.send(Command::Terminate);
        sleep(Duration::from_millis(100)).await;
    });
    return _main_thread;
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
