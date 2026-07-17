use bevy::prelude::*;

use crate::{GameEvent, UiEvent};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvent>().add_event::<UiEvent>();
    }
}
