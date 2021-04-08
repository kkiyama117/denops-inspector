use std::{io, task::Poll};

use anyhow::anyhow;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use futures::executor::block_on;
use futures::{Future, Sink};
use futures_util::{StreamExt, TryStreamExt};
use js_sys::Uint8Array;
use std::any::Any;
use std::fmt;
use std::mem;
use std::panic;
pub use std::thread::{current, sleep, Result, Thread};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Blob, Worker, WorkerOptions};

pub mod fetch;
pub(crate) mod file;
#[macro_use]
pub mod logging;
pub mod ws_cli;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = global, js_name = sendMessage)]
    fn js_send_message(message: JsValue);
}

/// Inner representation for JoinHandle
struct JoinInner<T> {
    // thread: Thread,
    receiver: UnboundedReceiver<T>,
}

impl<T> JoinInner<T> {
    fn join(&mut self) -> Result<T> {
        let res = block_on(self.receiver.recv().await);
        res.map(|t| t.unwrap())
            .map_err(|e| Box::new(e) as Box<(dyn Any + Send + 'static)>)
    }

    async fn join_async(&mut self) -> Result<T> {
        let res = self.receiver.recv().await;
        res.map_err(|e| Box::new(e) as Box<(dyn Any + Send + 'static)>)
            .await
    }
}

/// An owned permission to join on a thread (block on its termination).
pub struct JoinHandle<T>(JoinInner<T>);

impl<T> JoinHandle<T> {
    /// Extracts a handle to the underlying thread.
    pub fn thread(&self) -> &Thread {
        unimplemented!();
        //&self.0.thread
    }

    /// Waits for the associated thread to finish.
    pub fn join(mut self) -> Result<T> {
        self.0.join()
    }

    /// Waits for the associated thread to finish asynchronously.
    pub async fn join_async(mut self) -> Result<T> {
        self.0.join_async().await
    }
}

impl<T> fmt::Debug for JoinHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("JoinHandle { .. }")
    }
}
pub fn spawn<T>(task: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    let (tx, rx) = unbounded::<T::Output>();
    JoinHandle {
        0: JoinInner { receiver: rx },
    }
}

// /// Spawns a new thread, returning a JoinHandle for it.
// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
//     where
//         F: FnOnce() -> T,
//         F: Send + 'static,
//         T: Send + 'static,
// {
//     Builder::new().spawn(f).expect("failed to spawn thread")
// }

// #[derive(Clone)]
// struct MessageWriter;
//
// impl Sink<String> for MessageWriter {
//     type Error = io::Error;
//
//     fn poll_ready(
//         self: std::pin::Pin<&mut Self>,
//         _cx: &mut std::task::Context<'_>,
//     ) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }
//
//     fn start_send(self: std::pin::Pin<&mut Self>, item: String) -> Result<(), Self::Error> {
//         log_debug!("out: {}", serde_json::to_string(&item).unwrap());
//         js_send_message(JsValue::from_serde(&item).unwrap());
//         Ok(())
//     }
//
//     fn poll_flush(
//         self: std::pin::Pin<&mut Self>,
//         _cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }
//
//     fn poll_close(
//         self: std::pin::Pin<&mut Self>,
//         _cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }
// }
