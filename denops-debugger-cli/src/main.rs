use denops_debugger_core::client::{DebuggerClient, DebuggerClientTrait};
use denops_debugger_core::external::*;

#[tokio::main]
async fn main() {
    let dc = DebuggerClient::default();
    dbg!(dc.check_version().await);

    let a = dc.get_worker_list().await;
    println!("{}", &a.get(0).unwrap());
    ws_cli::ws_connection(String::from(&a.get(0).unwrap().web_socket_debugger_url));
}
