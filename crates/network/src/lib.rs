use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkConfig::default());
    }
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub enabled: bool,
    pub server_port: u16,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            server_port: 7777,
        }
    }
}
