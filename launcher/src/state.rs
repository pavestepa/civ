use civ_channel::WireMessage;
use civ_ui_bridge::{IpcEnvelope, IpcMessage, UiChannelEndpoint};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub struct LauncherState {
    pub channel: Mutex<UiChannelEndpoint>,
}

impl LauncherState {
    pub fn new(channel: UiChannelEndpoint) -> Self {
        Self {
            channel: Mutex::new(channel),
        }
    }

    pub fn forward_event(
        &self,
        op: impl Into<String>,
        body: serde_json::Value,
    ) -> Result<(), String> {
        static SEQ: AtomicU64 = AtomicU64::new(0);
        let id = SEQ.fetch_add(1, Ordering::Relaxed);
        let envelope = IpcEnvelope::new(
            id,
            IpcMessage::Event {
                op: op.into(),
                body,
            },
        );
        self.channel
            .lock()
            .map_err(|e| e.to_string())?
            .send(envelope);
        Ok(())
    }

    pub fn forward_request(
        &self,
        id: u64,
        op: impl Into<String>,
        body: serde_json::Value,
    ) -> Result<(), String> {
        let envelope = IpcEnvelope::new(
            id,
            IpcMessage::Request {
                id,
                op: op.into(),
                body,
            },
        );
        self.channel
            .lock()
            .map_err(|e| e.to_string())?
            .send(envelope);
        Ok(())
    }

    pub fn poll_outbox(&self) -> Result<Vec<WireMessage>, String> {
        let channel = self.channel.lock().map_err(|e| e.to_string())?;
        let mut messages = Vec::new();

        while let Some(envelope) = channel.try_recv() {
            match envelope.message {
                IpcMessage::Response { id, op, body } => {
                    messages.push(WireMessage::res(op, id, body));
                }
                IpcMessage::Event { op, body } => {
                    messages.push(WireMessage::evt(op, body));
                }
                IpcMessage::Request { .. } => {}
            }
        }

        Ok(messages)
    }
}

#[derive(Clone)]
pub struct LauncherIpc(pub Arc<LauncherState>);

impl LauncherIpc {
    pub fn new(channel: UiChannelEndpoint) -> Self {
        Self(Arc::new(LauncherState::new(channel)))
    }

    pub fn poll_outbox(&self) -> Result<Vec<WireMessage>, String> {
        self.0.poll_outbox()
    }
}
