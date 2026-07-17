use bevy::prelude::*;
use civ_ui_bridge::UiChannel;
use tracing_subscriber::EnvFilter;

use crate::plugins::EnginePluginGroup;

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}

/// Builds a Bevy app with the shared UI channel injected.
pub fn build_app(ui_channel: UiChannel) -> App {
    init_tracing();

    let mut app = App::new();
    app.insert_resource(ui_channel);
    app.add_plugins(EnginePluginGroup);
    app
}

/// Standalone engine entry (no Tauri overlay, isolated IPC channel).
pub fn run() {
    build_app(UiChannel::default()).run();
}
