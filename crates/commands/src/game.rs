use bevy::prelude::*;
use civ_common::{PlayerId, UnitId};
use civ_hex::HexCoordinate;
use serde::{Deserialize, Serialize};

/// Commands that mutate simulation state. Processed by the Rust API layer.
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub enum GameCommand {
    EndTurn,
    SpawnUnit {
        owner: PlayerId,
        at: HexCoordinate,
        kind: String,
    },
    CreateCity {
        owner: PlayerId,
        at: HexCoordinate,
        name: String,
    },
    AddGold {
        player: PlayerId,
        amount: i32,
    },
    MoveUnit {
        unit: UnitId,
        to: HexCoordinate,
    },
    SelectEntity {
        kind: String,
        id: Option<String>,
    },
    FindCity {
        name: String,
    },
    GetPlayer {
        id: PlayerId,
    },
}
