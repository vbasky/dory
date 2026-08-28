//! GPUI-coupled keymap types for Dory.
//!
//! This module contains keymap types that depend on GPUI:
//! - `actions` — GPUI action definitions
//! - `dispatcher` — Command dispatcher trait
//! - `defaults` — Default keymap bindings
//! - `chord_ext` — GPUI keystroke conversion utilities

mod actions;
mod dispatcher;

// Re-export pure keymap types from dory_app
pub use dory_app::keymap::{
    Command, ContextId, FocusTarget, KeyChord, KeymapLayer, KeymapStack, Modifiers,
};

#[allow(unused_imports)]
pub use dory_app::keymap::ParseError;

// Re-export Cancel from the component crate
pub use dory_components::actions::Cancel;

// GPUI-coupled types that stay in dory_ui
pub use actions::*;
pub use dispatcher::CommandDispatcher;

// Keymap helpers re-exported from dory_ui_base
pub use dory_ui_base::keymap::default_keymap;
pub use dory_ui_base::keymap::key_chord_from_gpui;
