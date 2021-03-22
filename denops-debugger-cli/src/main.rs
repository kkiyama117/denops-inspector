use denops_debugger_core::external::*;
use denops_debugger_core::{DebuggerClient, DebuggerClientTrait};
use v8_inspector_api_types::browser::Version;
use v8_inspector_api_types::prelude::WebSocketConnectionInfo;

#[tokio::main]
async fn main() {
    let dc = DebuggerClient::new();
    dbg!(dc.check_version().await);

    let a = dc.get_worker_list().await;
    println!("{}", &a.get(0).unwrap());
    ws_cli::ws_connection(String::from(&a.get(0).unwrap().web_socket_debugger_url));
}
