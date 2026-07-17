//! Hexagonal coordinate math (axial / cube representation).
//!
//! Renderer-independent. Used by world simulation and pathfinding.

pub mod coordinate;
pub mod layout;

pub use coordinate::HexCoordinate;
pub use layout::HexLayout;
