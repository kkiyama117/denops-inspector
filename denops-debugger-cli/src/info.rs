use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Info {
    pub base_url: Url,
}

impl Info {
    pub fn new(base_url: Url) -> Self {
        Self { base_url }
    }
}

impl FromStr for Info {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Url::parse(s) {
            Ok(base_url) => Ok(Self::new(base_url)),
            Err(e) => Err(e),
        }
    }
}
