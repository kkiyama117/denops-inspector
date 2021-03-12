use futures::channel::mpsc;
use futures_util::lock::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::Arc;
use v8_inspector_api_types::prelude::*;
use ws::{connect, CloseCode};

// #[derive(Debug)]
// pub struct Transport {
//     web_socket_connection: Arc<WebSocketConnection>,
//     waiting_call_registry: Arc<WaitingCallRegistry>,
//     listeners: Listeners,
//     open: Arc<AtomicBool>,
//     call_id_counter: Arc<AtomicU32>,
//     loop_shutdown_tx: Mutex<mpsc::Sender<()>>,
// }
//
// impl Transport {
//     pub fn call_method<C>(
//         &self,
//         method: C,
//         destination: MethodDestination,
//     ) -> Fallible<C::ReturnObject>
//     where
//         C: protocol::Method + serde::Serialize,
//     {
//     }
// }

pub fn ws_connection(url: String) {
    let data = r#"{"method": "Debugger.enable","params": null, "id": 1}"#;
    // let data = debugger::methods::Enable::to_method_call();
    connect(url, |out| {
        out.send(data).unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            // out.close(CloseCode::Normal)
            Ok(())
        }
    })
    .unwrap()
}
