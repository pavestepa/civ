use crate::HexCoordinate;

/// Pointy-top hex layout parameters for world-to-screen projection.
#[derive(Debug, Clone, Copy)]
pub struct HexLayout {
    pub size: f32,
}

pub const DEFAULT_HEX_SIZE: f32 = 1.0;
pub const TILE_CIRCUMRADIUS: f32 = 0.92;
pub const GRID_OUTER_RADIUS: f32 = 0.94;
pub const GRID_INNER_RADIUS: f32 = 0.88;
pub const SELECTION_OUTER_RADIUS: f32 = 0.94;
pub const SELECTION_INNER_RADIUS: f32 = 0.82;

impl Default for HexLayout {
    fn default() -> Self {
        Self {
            size: DEFAULT_HEX_SIZE,
        }
    }
}

impl HexLayout {
    pub fn world_position(&self, coord: HexCoordinate) -> glam::Vec3 {
        coord.to_world_position(self.size)
    }

    pub fn terrain_surface(&self, coord: HexCoordinate, elevation: i16) -> glam::Vec3 {
        self.world_position(coord) + glam::Vec3::Y * (0.02 + elevation as f32 * 0.05)
    }

    pub fn grid_overlay(&self, coord: HexCoordinate, elevation: i16) -> glam::Vec3 {
        self.terrain_surface(coord, elevation) + glam::Vec3::Y * 0.04
    }

    pub fn unit_base(&self, coord: HexCoordinate, elevation: i16) -> glam::Vec3 {
        self.terrain_surface(coord, elevation) + glam::Vec3::Y * 0.3
    }

    pub fn city_base(&self, coord: HexCoordinate, elevation: i16) -> glam::Vec3 {
        self.terrain_surface(coord, elevation) + glam::Vec3::Y * 0.2
    }
}
