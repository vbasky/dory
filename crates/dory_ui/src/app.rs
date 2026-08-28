//! Compatibility module re-exporting types from dory_app.
//!
//! This module exists to ease the transition of UI code that previously
//! used `crate::app::AppState` when it was in the dory crate.
//! New code should use `dory_app::AppState` directly or `AppStateEntity`
//! from the parent crate.

pub use dory_app::AppState;
pub use dory_app::{ExternalDriverDiagnostic, ExternalDriverStage};
pub use dory_core::ConnectedProfile;

// Re-export event types from dory_ui_base
#[cfg(feature = "mcp")]
pub use dory_ui_base::McpRuntimeEventRaised;
pub use dory_ui_base::{AppStateChanged, AppStateEntity, AuthProfileCreated};
