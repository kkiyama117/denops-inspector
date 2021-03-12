use denops_debugger_core::external::*;
use v8_inspector_api_types::browser::Version;
use v8_inspector_api_types::prelude::WebSocketConnectionInfo;

#[tokio::main]
async fn main() {
    dbg!(
        fetch::fetch::<Version>("http://localhost:9229/json/version")
            .await
            .unwrap()
    );
    let a = fetch::fetch::<Vec<WebSocketConnectionInfo>>("http://localhost:9229/json")
        .await
        .unwrap()
        .get(0)
        .unwrap()
        .clone()
        .web_socket_debugger_url;
    println!("{}", &a);
    ws_cli::ws_connection(a);
}
