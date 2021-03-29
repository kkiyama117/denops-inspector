use std::fmt::{Display, Formatter, Result};

use crate::types::JsInt;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Deserialize, Debug, PartialEq, Clone)]
pub struct RemoteError {
    pub code: JsInt,
    pub message: String,
}

impl Display for RemoteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Method call error {}: {}", self.code, self.message)
    }
}
