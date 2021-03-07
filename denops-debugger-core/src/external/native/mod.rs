pub (crate) mod ws_cli;
pub (crate) mod logging;
pub (crate) mod file;

use futures::Future;

pub(crate) fn spawn<F: Future<Output=()> + Send + 'static>(fut: F) {
    tokio::spawn(fut);
}

// pub(crate) fn needs_update(p: &str, new_date_ms: u64) -> Result<bool, anyhow::Error> {
//     Ok(std::fs::metadata(p)?
//         .modified()?
//         .duration_since(UNIX_EPOCH)?
//         .as_millis()
//         < new_date_ms as u128)
// }
