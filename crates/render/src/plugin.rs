use bevy::prelude::*;
use civ_channel::UiInputSet;

use crate::{
    camera::spawn_camera,
    camera_controller::{init_map_camera_controller, update_map_camera_controller},
    lighting::spawn_lighting,
    meshes::attach_example_meshes,
    scene::load_scene_assets,
    selection::{handle_map_click, SelectionState},
};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 150.0,
        })
        .init_resource::<SelectionState>()
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_lighting,
                load_scene_assets,
                init_map_camera_controller,
            ),
        )
        .add_systems(PostStartup, attach_example_meshes)
        .add_systems(
            Update,
            (
                update_map_camera_controller,
                handle_map_click,
            )
                .chain()
                .after(UiInputSet),
        );
    }
}
