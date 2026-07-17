use bevy::prelude::*;

use crate::{GameCommand, UiCommand};

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameCommand>().add_event::<UiCommand>();
    }
}
