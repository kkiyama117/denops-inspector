use async_trait::async_trait;
use v8_inspector_api_types::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "external/native/mod.rs"]
pub mod external;

pub mod client;
#[cfg(target_arch = "wasm32")]
#[path = "external/deno/mod.rs"]
pub mod external;
