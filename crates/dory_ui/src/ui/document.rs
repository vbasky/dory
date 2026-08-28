//! Compat shim: the document subsystem now lives in `dory_ui_document`.
pub use dory_ui_document::*;
// workspace/render.rs:839 uses crate::ui::document::tab_bar::TabBar
pub use dory_ui_document::tab_bar;
