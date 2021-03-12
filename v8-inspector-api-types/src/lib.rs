// TODO: Add feature to use only specific mod.
pub mod browser;
pub mod websocket_target_list;

pub mod prelude {
    pub use crate::browser::*;
    pub use crate::websocket_target_list::*;
    pub use headless_chrome::protocol::*;
}
