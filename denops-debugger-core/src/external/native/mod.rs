use futures::Future;

pub mod fetch;
pub(crate) mod file;
pub mod logging;
pub mod ws;
pub mod ws_cli;
pub(crate) mod ws_cli2;

pub(crate) fn spawn<F: Future<Output = ()> + Send + 'static>(fut: F) {
    tokio::spawn(fut);
}
