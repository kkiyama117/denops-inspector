#[cfg(not(target_arch = "wasm32"))]
#[path = "external/native/mod.rs"]
#[macro_use]
pub mod external;

#[cfg(target_arch = "wasm32")]
#[path = "external/deno/mod.rs"]
#[macro_use]
pub mod external;

pub mod v8_types;
