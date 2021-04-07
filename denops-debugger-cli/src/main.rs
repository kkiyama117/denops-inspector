use denops_debugger_core::client::HTTPManager;
use denops_debugger_core::{
    client::Manager,
    ws_manager::{TestMsg, WebSocketManager},
};
use futures::channel::mpsc::{channel, Sender};
use futures::future::BoxFuture;
use futures::join;
use futures_util::{FutureExt, SinkExt};
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio::time::{sleep, Duration};
use v8_inspector_api_types::http_methods::WebSocketConnectionInfo;
use v8_inspector_api_types::{methods::Method, protocols::debugger::methods};

#[tokio::main]
async fn main() {
    let InitializedValue {
        mut stx,
        mut tx,
        ws_manager: b,
    }: InitializedValue = initialize().await;

    let main_thread = async move {
        println!("Start!");
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
                        tx.send(TestMsg::Msg(data)).await.unwrap();
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
        stx.send(true).await.unwrap();
        tx.send(TestMsg::Terminate).await.unwrap();
        sleep(Duration::from_millis(100)).await;
    };
    let _ = join!(b.reader, b.writer, main_thread);
}

async fn initialize() -> InitializedValue {
    let (stx, srx) = channel::<bool>(1);
    let (tx, rx) = channel::<TestMsg>(10);

    let man = Manager::from_string("http://localhost:9229").unwrap();
    let stream = man.get_ws_cli(selector).await.unwrap();
    let ws_manager = WebSocketManager::new(stream, rx, srx);
    InitializedValue {
        stx,
        tx,
        ws_manager,
    }
}

fn selector(
    x: Vec<WebSocketConnectionInfo>,
) -> BoxFuture<'static, Option<WebSocketConnectionInfo>> {
    async move { x.get(1).cloned() }.boxed()
}

struct InitializedValue {
    stx: Sender<bool>,
    tx: Sender<TestMsg>,
    ws_manager: WebSocketManager,
}
