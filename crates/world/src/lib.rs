//! Pure simulation world model.
//!
//! Contains no Bevy, React, Tauri, or QuickJS dependencies.
//! The renderer observes this data through ECS adapters.

pub mod city;
pub mod components;
pub mod generation;
pub mod player;
pub mod tile;
pub mod unit;
pub mod world;

pub use city::City;
pub use components::{Elevation, Owner, ResourceDeposit, River, Road, TerrainKind, Visibility};
pub use generation::{is_land, terrain_at};
pub use player::Player;
pub use tile::Tile;
pub use unit::Unit;
pub use world::{WorldMap, DEFAULT_MAP_HEIGHT, DEFAULT_MAP_WIDTH};
