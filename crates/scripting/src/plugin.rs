use bevy::prelude::*;

use crate::{api::ScriptGameApi, runtime::ScriptRuntime};

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScriptRuntime::default())
            .insert_resource(ScriptGameApi::default());
    }
}
