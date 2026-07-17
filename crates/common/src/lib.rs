//! Shared foundation types used across all layers.
//!
//! This crate has no dependency on Bevy, React, Tauri, or QuickJS.

pub mod error;
pub mod ids;
pub mod state;
pub mod version;

pub use error::{CivError, CivResult};
pub use ids::{CityId, EntityId, PlayerId, UnitId};
pub use state::AppState;
pub use version::{GameVersion, SAVE_FORMAT_VERSION};
