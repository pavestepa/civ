use bevy::prelude::*;
use tracing_subscriber::EnvFilter;

use crate::plugins::EnginePluginGroup;

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    App::new()
        .add_plugins(EnginePluginGroup)
        .run();
}
