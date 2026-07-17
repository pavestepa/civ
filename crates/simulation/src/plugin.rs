use bevy::prelude::*;

use crate::{state::SimulationConfig, state::SimulationState, systems::process_commands};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationState::default())
            .insert_resource(SimulationConfig::default())
            .add_systems(Update, process_commands);
    }
}
