use bevy::prelude::*;

use crate::TerrainGenerator;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainGenerator::new(42));
    }
}
