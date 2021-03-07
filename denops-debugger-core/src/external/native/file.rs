use std::{path::Path, };

pub(crate) fn is_absolute_path(p: &str) -> bool {
    Path::new(p).is_absolute()
}

pub(crate) async fn read_file(p: &str) -> Result<Vec<u8>, anyhow::Error> {
    Ok(tokio::fs::read(p).await?)
}

pub(crate) async fn write_file(p: &str, data: &[u8]) -> Result<(), anyhow::Error> {
    Ok(tokio::fs::write(p, data).await?)
}

pub(crate) fn file_exists(p: &str) -> bool {
    Path::new(p).exists()
}

pub(crate) fn mkdir(p: &str) -> Result<(), anyhow::Error> {
    std::fs::create_dir_all(p)?;
    Ok(())
}
