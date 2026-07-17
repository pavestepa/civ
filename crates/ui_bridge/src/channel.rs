use bevy::prelude::Resource;
use crossbeam_channel::{Receiver, Sender, unbounded};
use std::sync::{Arc, Mutex};

use crate::protocol::IpcEnvelope;

/// Thread-safe IPC channel between Tauri and Bevy engine.
#[derive(Clone, Resource)]
pub struct UiChannel {
    pub to_engine: Sender<IpcEnvelope>,
    pub from_engine: Arc<Mutex<Receiver<IpcEnvelope>>>,
}

impl Default for UiChannel {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self {
            to_engine: tx,
            from_engine: Arc::new(Mutex::new(rx)),
        }
    }
}

impl UiChannel {
    pub fn paired() -> (Self, UiChannelEndpoint) {
        let (to_engine_tx, to_engine_rx) = unbounded();
        let (from_engine_tx, from_engine_rx) = unbounded();

        let engine_side = Self {
            to_engine: to_engine_tx,
            from_engine: Arc::new(Mutex::new(from_engine_rx)),
        };

        let launcher_side = UiChannelEndpoint {
            to_engine: from_engine_tx,
            from_engine: to_engine_rx,
        };

        (engine_side, launcher_side)
    }

    pub fn send_to_engine(&self, envelope: IpcEnvelope) {
        let _ = self.to_engine.send(envelope);
    }
}

#[derive(Clone)]
pub struct UiChannelEndpoint {
    pub to_engine: Sender<IpcEnvelope>,
    pub from_engine: Receiver<IpcEnvelope>,
}

impl UiChannelEndpoint {
    pub fn send(&self, envelope: IpcEnvelope) {
        let _ = self.to_engine.send(envelope);
    }

    pub fn try_recv(&self) -> Option<IpcEnvelope> {
        self.from_engine.try_recv().ok()
    }
}
