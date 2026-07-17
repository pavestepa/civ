use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PathfindingState {
    pub _enabled: bool,
}

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PathfindingState::default());
    }
}
