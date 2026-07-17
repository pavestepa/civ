use civ_ui_bridge::{IpcEnvelope, IpcMessage, UiChannelEndpoint};
use std::sync::Mutex;

pub struct LauncherState {
    pub channel: Mutex<UiChannelEndpoint>,
    pub next_id: Mutex<u64>,
}

impl Default for LauncherState {
    fn default() -> Self {
        let (_engine, launcher) = civ_ui_bridge::UiChannel::paired();
        Self {
            channel: Mutex::new(launcher),
            next_id: Mutex::new(0),
        }
    }
}

impl LauncherState {
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
