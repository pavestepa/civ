use civ_hex::HexCoordinate;
use civ_world::components::TerrainKind;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use bevy::prelude::*;

#[derive(Resource)]
pub struct TerrainGenerator {
    perlin: Perlin,
    seed: u32,
}

impl TerrainGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            seed,
        }
    }

    pub fn terrain_at(&self, coord: HexCoordinate) -> TerrainKind {
        let nx = coord.q as f64 * 0.1;
        let ny = coord.r as f64 * 0.1;
        let value = self.perlin.get([nx, ny]);

        if value < -0.2 {
            TerrainKind::Ocean
        } else if value < 0.0 {
            TerrainKind::Coast
        } else if value < 0.3 {
            TerrainKind::Plains
        } else if value < 0.5 {
            TerrainKind::Grassland
        } else if value < 0.7 {
            TerrainKind::Hills
        } else {
            TerrainKind::Mountain
        }
    }

    pub fn elevation_at(&self, coord: HexCoordinate) -> i16 {
        let mut rng = rand::rng();
        let _ = self.seed;
        let base = self.perlin.get([coord.q as f64, coord.r as f64]);
        (base * 100.0) as i16 + rng.random_range(0..3)
    }
}
