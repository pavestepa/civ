use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetPaths>();
    }
}

#[derive(Resource, Debug, Default)]
pub struct AssetPaths {
    pub root: String,
}

impl AssetPaths {
    pub fn new(root: impl Into<String>) -> Self {
        Self { root: root.into() }
    }
}
