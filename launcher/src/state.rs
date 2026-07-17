use civ_ui_bridge::{IpcEnvelope, IpcMessage, UiChannelEndpoint};
use std::sync::{Arc, Mutex};

pub struct LauncherState {
    pub channel: Mutex<UiChannelEndpoint>,
    pub next_id: Mutex<u64>,
}

impl LauncherState {
    pub fn new(channel: UiChannelEndpoint) -> Self {
        Self {
            channel: Mutex::new(channel),
            next_id: Mutex::new(0),
        }
    }

    pub fn next_id(&self) -> u64 {
        let mut id = self.next_id.lock().expect("next_id mutex poisoned");
        *id += 1;
        *id
    }

    pub fn send(&self, message: IpcMessage) -> Result<u64, String> {
        let id = self.next_id();
        let envelope = IpcEnvelope::new(id, message);
        self.channel
            .lock()
            .map_err(|e| e.to_string())?
            .send(envelope);
        Ok(id)
    }

    pub fn poll(&self) -> Result<Vec<String>, String> {
        let channel = self.channel.lock().map_err(|e| e.to_string())?;
        let mut events = Vec::new();
        while let Some(envelope) = channel.try_recv() {
            if let Ok(json) = envelope.to_json() {
                events.push(json);
            }
        }
        Ok(events)
    }
}

/// Shared launcher IPC state accessible from Bevy systems and the wry IPC handler.
#[derive(Clone)]
pub struct LauncherIpc(pub Arc<LauncherState>);

impl LauncherIpc {
    pub fn new(channel: UiChannelEndpoint) -> Self {
        Self(Arc::new(LauncherState::new(channel)))
    }

    pub fn poll(&self) -> Result<Vec<String>, String> {
        self.0.poll()
    }
}
