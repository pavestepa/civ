use bevy::prelude::*;
use civ_world::WorldMap;
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    pub world: WorldMap,
    pub seed: u64,
    pub deterministic: bool,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            world: WorldMap::with_example_entities(),
            seed: 42,
            deterministic: true,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct SimulationConfig {
    pub ticks_per_turn: u32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self { ticks_per_turn: 1 }
    }
}
