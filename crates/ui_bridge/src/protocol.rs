use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IpcMessage {
    Request {
        id: u64,
        op: String,
        body: Value,
    },
    Response {
        id: u64,
        op: String,
        body: Value,
    },
    Event {
        op: String,
        body: Value,
    },
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

#[derive(Resource, Default)]
pub struct PendingResponses {
    pub queue: Vec<(u64, String, Value)>,
}

impl PendingResponses {
    pub fn push(&mut self, id: u64, op: impl Into<String>, body: Value) {
        self.queue.push((id, op.into(), body));
    }
}
