// pub(crate) mod external;

#[cfg(target_arch = "wasm32")]
#[path = "external/deno/mod.rs"]
#[macro_use]
pub(crate) mod external;

pub mod errors;
