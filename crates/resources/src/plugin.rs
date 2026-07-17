use bevy::prelude::*;

use crate::registry::GameDataRegistry;

pub struct GameResourcesPlugin;

impl Plugin for GameResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameDataRegistry::with_defaults());
    }
}
