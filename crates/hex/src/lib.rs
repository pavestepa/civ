//! Hexagonal coordinate math (axial / cube representation).
//!
//! Renderer-independent. Used by world simulation and pathfinding.

pub mod coordinate;
pub mod grid;
pub mod layout;

pub use coordinate::HexCoordinate;
pub use grid::rect_coordinates;
pub use layout::{
    HexLayout, DEFAULT_HEX_SIZE, GRID_INNER_RADIUS, GRID_OUTER_RADIUS, SELECTION_INNER_RADIUS,
    SELECTION_OUTER_RADIUS, TILE_CIRCUMRADIUS,
};
