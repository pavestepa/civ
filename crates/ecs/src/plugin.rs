use bevy::prelude::*;

use crate::sync::sync_example_entities;

pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sync_example_entities);
    }
}
