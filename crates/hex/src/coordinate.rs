use glam::Vec3;
use serde::{Deserialize, Serialize};

/// Axial hex coordinate (q, r). Cube s is derived: s = -q - r.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct HexCoordinate {
    pub q: i32,
    pub r: i32,
}

impl HexCoordinate {
    pub const ZERO: Self = Self { q: 0, r: 0 };

    pub const fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub const fn s(self) -> i32 {
        -self.q - self.r
    }

    pub fn neighbors(self) -> [Self; 6] {
        const DIRS: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
        DIRS.map(|(dq, dr)| Self::new(self.q + dq, self.r + dr))
    }

    pub fn distance(self, other: Self) -> u32 {
        let dq = (self.q - other.q).unsigned_abs();
        let dr = (self.r - other.r).unsigned_abs();
        let ds = (self.s() - other.s()).unsigned_abs();
        dq.max(dr).max(ds)
    }

    pub fn to_world_position(self, size: f32) -> Vec3 {
        let x = size * (3.0_f32.sqrt() * self.q as f32 + 3.0_f32.sqrt() / 2.0 * self.r as f32);
        let z = size * (1.5 * self.r as f32);
        Vec3::new(x, 0.0, z)
    }
}
