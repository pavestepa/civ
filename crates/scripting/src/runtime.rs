use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ScriptRuntime {
    pub loaded_scripts: Vec<String>,
}

impl ScriptRuntime {
    pub fn register_script(&mut self, path: impl Into<String>) {
        self.loaded_scripts.push(path.into());
    }
}
