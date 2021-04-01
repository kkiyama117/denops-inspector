use futures::Future;

pub mod fetch;
pub(crate) mod file;
#[macro_use]
pub mod logging;
pub mod ws_cli;

pub fn spawn<T>(task: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::spawn(task)
}

pub(crate) type JoinHandle<T> = tokio::task::JoinHandle<T>;
