use bevy::prelude::*;
use civ_channel::{engine_event::EngineEventOutbox, front_api::FrontApiRegistry, UiInputSet, WebviewInputState};

use crate::{
    channel::UiChannel,
    systems,
    PendingResponses,
};

pub struct UiBridgePlugin;

impl Plugin for UiBridgePlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<UiChannel>() {
            app.insert_resource(UiChannel::default());
        }

        app.init_resource::<FrontApiRegistry>()
            .init_resource::<EngineEventOutbox>()
            .init_resource::<PendingResponses>()
            .init_resource::<WebviewInputState>()
            .configure_sets(Update, UiInputSet)
            .add_systems(
                Update,
                (
                    systems::poll_ui_requests.in_set(UiInputSet),
                    systems::dispatch_ui_commands,
                    systems::flush_command_responses,
                    systems::forward_game_events_to_ui,
                    systems::flush_engine_events,
                )
                    .chain(),
            );
    }
}
