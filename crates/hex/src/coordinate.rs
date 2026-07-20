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

    /// Inverse of [`Self::to_world_position`] for the XZ plane (Y is ignored).
    pub fn from_world_position(pos: Vec3, size: f32) -> Self {
        let q = (3.0_f32.sqrt() / 3.0 * pos.x - 1.0 / 3.0 * pos.z) / size;
        let r = (2.0 / 3.0 * pos.z) / size;
        Self::round_axial(q, r)
    }

    /// Round fractional axial coordinates to the nearest hex.
    pub fn round_axial(q: f32, r: f32) -> Self {
        let s = -q - r;
        let mut rq = q.round();
        let mut rr = r.round();
        let rs = s.round();

        let q_diff = (rq - q).abs();
        let r_diff = (rr - r).abs();
        let s_diff = (rs - s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            rq = -rr - rs;
        } else if r_diff > s_diff {
            rr = -rq - rs;
        }

        Self::new(rq as i32, rr as i32)
    }

    pub fn contains_in_rect(self, width: u32, height: u32) -> bool {
        self.q >= 0 && self.r >= 0 && self.q < width as i32 && self.r < height as i32
    }
}
