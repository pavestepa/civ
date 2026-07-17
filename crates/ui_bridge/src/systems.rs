use bevy::prelude::*;
use civ_commands::{GameCommand, UiCommand};
use civ_events::{GameEvent, UiEvent};

use crate::channel::UiChannel;
use crate::protocol::{IpcEnvelope, IpcMessage};

pub fn poll_ui_commands(
    channel: Res<UiChannel>,
    mut ui_commands: EventWriter<UiCommand>,
) {
    let receiver = channel.from_engine.lock().expect("ui channel mutex poisoned");
    while let Ok(envelope) = receiver.try_recv() {
        if let IpcMessage::Command(cmd) = envelope.message {
            ui_commands.send(cmd);
        }
    }
}

pub fn dispatch_ui_commands(
    mut ui_commands: EventReader<UiCommand>,
    mut game_commands: EventWriter<GameCommand>,
    mut ui_events: EventWriter<UiEvent>,
) {
    for command in ui_commands.read() {
        match command {
            UiCommand::Dispatch(game_cmd) => {
                game_commands.send(game_cmd.clone());
            }
            UiCommand::RequestStateSnapshot => {
                ui_events.send(UiEvent::Notification {
                    message: "state snapshot requested".into(),
                });
            }
            UiCommand::SetAppState { state } => {
                ui_events.send(UiEvent::GameStateChanged { state: state.clone() });
            }
        }
    }
}

pub fn forward_game_events_to_ui(
    channel: Res<UiChannel>,
    mut game_events: EventReader<GameEvent>,
    mut ui_events: EventWriter<UiEvent>,
) {
    let mut id = 0u64;
    for event in game_events.read() {
        match event {
            GameEvent::TurnStarted { turn, active_player } => {
                ui_events.send(UiEvent::TurnChanged {
                    turn: *turn,
                    active_player: active_player.0,
                });
            }
            GameEvent::GoldChanged { player, total, .. } => {
                ui_events.send(UiEvent::PlayerGoldChanged {
                    player: player.0,
                    gold: *total,
                });
            }
            _ => {}
        }

        let envelope = IpcEnvelope::new(
            id,
            IpcMessage::Event(UiEvent::Notification {
                message: format!("{event:?}"),
            }),
        );
        channel.send_to_engine(envelope);
        id += 1;
    }
}
