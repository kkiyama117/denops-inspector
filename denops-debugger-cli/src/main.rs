use denops_debugger_core::external::*;
use denops_debugger_core::{DebuggerClient, DebuggerClientTrait};
use v8_inspector_api_types::commands;
use v8_inspector_api_types::commands::Method;

#[tokio::main]
async fn main() {
    let dc = DebuggerClient::new();
    dbg!(dc.check_version().await);

    let a = dc.get_worker_list().await;
    println!("{}", &a.get(0).unwrap());
    let a = commands::Enable {};
    let data = a.into_method_call(1);
    ws_cli::ws_connection((&a).get(0).unwrap().web_socket_debugger_url.to_string());
}
