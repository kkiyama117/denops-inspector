pub mod fetch;
pub(crate) mod file;
pub(crate) mod logging;
pub(crate) mod ws_cli;

use futures::Future;

pub(crate) fn spawn<F: Future<Output = ()> + Send + 'static>(fut: F) {
    tokio::spawn(fut);
}
