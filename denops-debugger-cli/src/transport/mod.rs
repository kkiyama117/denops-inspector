mod method_call_registry;

use crate::transport::method_call_registry::WaitingCallRegistry;
use crate::types::WsStream;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::{mpsc, Arc, Mutex};
use tokio::sync::watch::Sender;
use v8_inspector_api_types::messages::Event;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

type Listeners = Arc<Mutex<HashMap<SessionId, Sender<Event>>>>;

#[derive(Debug)]
pub struct Transport {
    web_socket_connection: WsStream,
    waiting_call_registry: Arc<WaitingCallRegistry>,
    listeners: Listeners,
    open: Arc<AtomicBool>,
    call_id_counter: Arc<AtomicU32>,
    loop_shutdown_tx: Mutex<mpsc::Sender<()>>,
}
