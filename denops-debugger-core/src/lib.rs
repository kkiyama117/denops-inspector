#[cfg(not(target_arch = "wasm32"))]
#[path = "external/native/mod.rs"]
pub mod external;

#[cfg(target_arch = "wasm32")]
#[path = "external/deno/mod.rs"]
pub mod external;
