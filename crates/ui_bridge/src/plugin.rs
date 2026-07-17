use bevy::prelude::*;

use crate::{channel::UiChannel, systems::{dispatch_ui_commands, forward_game_events_to_ui, poll_ui_commands}};

pub struct UiBridgePlugin;

impl Plugin for UiBridgePlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<UiChannel>() {
            app.insert_resource(UiChannel::default());
        }

        app.add_systems(
                Update,
                (
                    poll_ui_commands,
                    dispatch_ui_commands,
                    forward_game_events_to_ui,
                )
                    .chain(),
            );
    }
}
