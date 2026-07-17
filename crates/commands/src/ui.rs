use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::GameCommand;

/// Commands received from React via Tauri IPC.
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub enum UiCommand {
    Dispatch(GameCommand),
    RequestStateSnapshot,
    SetAppState { state: String },
}
