use bevy::prelude::*;

use crate::SaveService;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SaveService::default());
    }
}
