use bevy::prelude::Resource;
use serde::Serialize;

use crate::wire::WireMessage;

#[derive(Resource, Default)]
pub struct EngineEventOutbox {
    pub pending: Vec<WireMessage>,
}

impl EngineEventOutbox {
    pub fn emit<T: Serialize>(&mut self, op: &str, payload: T) {
        let body = serde_json::to_value(payload).unwrap_or(serde_json::Value::Null);
        self.pending.push(WireMessage::evt(op, body));
    }

    pub fn drain(&mut self) -> impl Iterator<Item = WireMessage> + '_ {
        self.pending.drain(..)
    }
}
