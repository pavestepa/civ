use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Events sent to the React UI layer via IPC.
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub enum UiEvent {
    GameStateChanged { state: String },
    TurnChanged { turn: u32, active_player: u32 },
    Notification { message: String },
    PlayerGoldChanged { player: u32, gold: i32 },
    SelectionChanged { kind: String, id: Option<String> },
}
