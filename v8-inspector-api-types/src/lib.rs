// TODO: Add feature to use only specific mod.
pub mod errors;
pub mod http_methods;
pub mod messages;
pub mod methods;
pub mod protocols;
pub mod types;

pub mod prelude {
    pub use crate::errors::*;
    pub use crate::http_methods::*;
    pub use crate::messages::*;
    pub use crate::methods::*;
    pub use crate::protocols::debugger::*;
}
