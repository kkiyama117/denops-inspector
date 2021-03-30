use tokio::io::AsyncWriteExt;

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        eprintln!($($arg)*)
    };
}

macro_rules! log_warn {
    ($($arg:tt)*) => {
        eprintln!($($arg)*)
    };
}

macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!($($arg)*)
    };
}

macro_rules! log_debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {  $crate::log_info!($($arg)*)  }
    };
}

async fn logging(a: &[u8]) -> tokio::io::Result<()> {
    tokio::io::stdout().write_all(a).await
}
