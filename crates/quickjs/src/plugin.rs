use bevy::prelude::*;
use civ_scripting::ScriptGameApi;

use crate::runtime::QuickJsRuntime;

#[derive(Resource, Default)]
pub struct QuickJsState {
    pub available: bool,
}

pub struct QuickJsPlugin;

impl Plugin for QuickJsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QuickJsState>();

        match QuickJsRuntime::new() {
            Ok(runtime) => {
                if let Some(api) = app.world().get_resource::<ScriptGameApi>() {
                    if let Err(err) = runtime.register_game_api(api) {
                        tracing::warn!("QuickJS API registration failed: {err}");
                    }
                }
                app.insert_non_send_resource(runtime);
                app.world_mut()
                    .resource_mut::<QuickJsState>()
                    .available = true;
            }
            Err(err) => tracing::warn!("QuickJS runtime unavailable: {err}"),
        }
    }
}
