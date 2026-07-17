use bevy::prelude::*;
use civ_common::{CityId, PlayerId, UnitId};
use civ_hex::HexCoordinate;
use serde::{Deserialize, Serialize};

/// Simulation-emitted events. Renderer and UI observe these indirectly.
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    TurnStarted { turn: u32, active_player: PlayerId },
    TurnEnded { turn: u32 },
    UnitSpawned { id: UnitId, owner: PlayerId, at: HexCoordinate },
    CityFounded { id: CityId, owner: PlayerId, at: HexCoordinate },
    GoldChanged { player: PlayerId, delta: i32, total: i32 },
    TileChanged { at: HexCoordinate },
}
