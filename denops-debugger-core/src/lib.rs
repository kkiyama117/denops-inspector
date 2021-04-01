#[cfg(not(target_arch = "wasm32"))]
#[path = "external/native/mod.rs"]
#[macro_use]
pub(crate) mod external;

#[cfg(target_arch = "wasm32")]
#[path = "external/deno/mod.rs"]
#[macro_use]
pub(crate) mod external;

pub mod client;
pub mod errors;
pub mod ws_manager;
