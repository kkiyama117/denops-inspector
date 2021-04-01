use denops_debugger_core::{
    client::get_ws_cli,
    client::Manager,
    ws_manager::{TestMsg, WebSocketManager},
};
use futures::channel::mpsc::channel;
use futures_util::SinkExt;
use tokio::time::{sleep, Duration};
use v8_inspector_api_types::{methods::Method, protocols::debugger::methods};

#[tokio::main]
async fn main() {
    let (stx, srx) = channel::<bool>(1);
    let (mut tx, rx) = channel::<TestMsg>(10);

    let man = Manager::from_string("http://localhost:9229").unwrap();
    let b = WebSocketManager::new(get_ws_cli(man).await.unwrap(), rx, srx);

    let main_thread = async move {
        let command = methods::Enable {};
        let data = command.into_method_call(1);
        let data = serde_json::to_string(data.as_ref()).unwrap();

        sleep(Duration::from_millis(1000)).await;
        tx.send(TestMsg::Msg(data)).await.unwrap();

        let command = methods::GetPossibleBreakpoints {};
        let data = command.into_method_call(2);
        let data = serde_json::to_string(data.as_ref()).unwrap();
        tx.send(TestMsg::Msg(data)).await.unwrap();
        sleep(Duration::from_millis(5000)).await;

        // stx.send(true).await.unwrap();
        // tx.send(TestMsg::Terminate).await.unwrap();
        // sleep(Duration::from_millis(1000)).await;
    };
    let _ = tokio::join!(b.reader, b.writer, main_thread);
    // run all threads.
    // match res {
    //     Ok((_, _, _)) => {
    //         println!("successfully finished");
    //     }
    //     Err(e) => {
    //         eprintln!("Error in thread=>\n{}", e);
    //     }
    // }
    // match res {
    //     Ok((_, _, _)) => {
    //         println!("successfully finished");
    //     }
    //     Err(e) => {
    //         eprintln!("Error in thread=>\n{}", e);
    //     }
    // }
}
