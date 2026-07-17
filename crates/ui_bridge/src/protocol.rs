use serde::{Deserialize, Serialize};

use civ_commands::{GameCommand, UiCommand};
use civ_events::UiEvent;

/// Wire protocol between React and Rust.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum IpcMessage {
    Command(UiCommand),
    Event(UiEvent),
    GameCommand(GameCommand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcEnvelope {
    pub id: u64,
    pub message: IpcMessage,
}

impl IpcEnvelope {
    pub fn new(id: u64, message: IpcMessage) -> Self {
        Self { id, message }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(value: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(value)
    }
}
