use crate::engine_event::EngineEventOutbox;

use super::model::{PlayerGoldChanged, OP};

pub fn emit(outbox: &mut EngineEventOutbox, payload: PlayerGoldChanged) {
    outbox.emit(OP, payload);
}
