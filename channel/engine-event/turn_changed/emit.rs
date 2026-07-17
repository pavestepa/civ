use crate::engine_event::EngineEventOutbox;

use super::model::{TurnChanged, OP};

pub fn emit(outbox: &mut EngineEventOutbox, payload: TurnChanged) {
    outbox.emit(OP, payload);
}
