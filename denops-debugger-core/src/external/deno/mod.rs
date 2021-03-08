pub mod fetch;
pub(crate) mod file;
#[macro_use]
pub mod logging;
pub mod ws_cli;

use anyhow::anyhow;
use futures::{Future, Sink};
use js_sys::Uint8Array;
use std::{io, task::Poll};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = global, js_name = sendMessage)]
    fn js_send_message(message: JsValue);
}

pub(crate) fn spawn<F: Future<Output = ()> + 'static>(fut: F) {
    spawn_local(fut)
}

struct ImplSend<T>(pub T);

// safety: we're in a WASM context with a single thread.
unsafe impl<T> Send for ImplSend<T> {}
unsafe impl<T> Sync for ImplSend<T> {}

// pub(crate) fn needs_update(p: &str, new_date_ms: u64) -> Result<bool, anyhow::Error> {
//     js_needs_update(p, new_date_ms).map_err(|e| anyhow!("{:?}", e))
// }

#[derive(Clone)]
struct MessageWriter;

impl Sink<String> for MessageWriter {
    type Error = io::Error;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: String) -> Result<(), Self::Error> {
        log_debug!("out: {}", serde_json::to_string(&item).unwrap());
        js_send_message(JsValue::from_serde(&item).unwrap());
        Ok(())
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}
