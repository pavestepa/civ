use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub title: String,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1600.0,
            height: 900.0,
            title: "Civ".into(),
        }
    }
}

pub struct WindowConfigPlugin;

impl Plugin for WindowConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowConfig::default());
    }
}
