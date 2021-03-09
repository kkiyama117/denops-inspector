// TODO: Add feature to use only specific mod.
pub mod browser;
pub mod http;

pub mod prelude {
    pub use crate::http::*;
}
