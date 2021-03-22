use futures::channel::mpsc;
use futures::lock::Mutex;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::Arc;
use tungstenite::WebSocket as WS;

#[derive(Debug)]
pub struct WebSocket {
    web_socket_connection: Arc<WS<TcpStream>>,
    // waiting_call_registry: Arc<WaitingCallRegistry>,
    // listeners: Listeners,
    open: Arc<AtomicBool>,
    call_id_counter: Arc<AtomicU32>,
    loop_shutdown_tx: Mutex<mpsc::Sender<()>>,
}
//
