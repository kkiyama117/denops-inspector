use futures::Future;

pub mod fetch;
pub(crate) mod file;
#[macro_use]
pub mod logging;
pub mod ws_cli;

pub(crate) fn spawn<F: Future<Output = ()> + Send + 'static>(fut: F) -> JoinHandle<()> {
    tokio::spawn(fut)
}

pub(crate) type JoinHandle<T> = tokio::task::JoinHandle<T>;
