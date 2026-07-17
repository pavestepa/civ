use crate::HexCoordinate;

/// Pointy-top hex layout parameters for world-to-screen projection.
#[derive(Debug, Clone, Copy)]
pub struct HexLayout {
    pub size: f32,
}

impl Default for HexLayout {
    fn default() -> Self {
        Self { size: 1.0 }
    }
}

impl HexLayout {
    pub fn world_position(&self, coord: HexCoordinate) -> glam::Vec3 {
        coord.to_world_position(self.size)
    }
}
