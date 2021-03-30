use denops_debugger_core::{
    client::get_ws_cli,
    client::Manager,
    ws_manager::{TestMsg, WebSocketManager},
};
use futures::{
    channel::mpsc::{channel, Receiver, Sender, TryRecvError},
    prelude::stream::{IntoStream, Next},
    try_join,
};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    task::JoinHandle,
    time::{sleep, Duration},
};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;
use v8_inspector_api_types::{messages::Message, methods::Method, protocols::debugger::methods};

#[tokio::main]
async fn main() {
    let (mut stx, mut srx) = channel::<bool>(1);
    let (mut tx, mut rx) = channel::<TestMsg>(10);

    let man = Manager::from_string("http://localhost:9229").unwrap();
    let mut b = WebSocketManager::new(get_ws_cli(man).await.unwrap(), rx, srx);

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
    // let res = try_join!(b.reader, b.writer, main_thread);
    // match res {
    //     Ok((_, _, _)) => {
    //         println!("successfully finished");
    //     }
    //     Err(e) => {
    //         eprintln!("Error in thread=>\n{}", e);
    //     }
    // }
}
