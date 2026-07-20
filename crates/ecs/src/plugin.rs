use bevy::prelude::*;

use crate::snap::snap_entities_to_hex_grid_system;
use crate::sync::sync_example_entities;

pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sync_example_entities)
            .add_systems(Update, snap_entities_to_hex_grid_system);
    }
}
