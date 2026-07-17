use bevy::prelude::*;

use crate::{
    camera::spawn_camera, lighting::spawn_lighting, meshes::attach_example_meshes,
    scene::load_scene_assets,
};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 150.0,
        })
        .add_systems(Startup, (spawn_camera, spawn_lighting, load_scene_assets))
            .add_systems(PostStartup, attach_example_meshes);
    }
}
