use bevy::prelude::*;
use civ_channel::{
    engine_event::EngineEventOutbox,
    front_api::{ApiContext, FrontApiRegistry},
    input_frame, player_gold_changed, turn_changed, INPUT_FRAME_OP, WebviewInputState,
};
use civ_commands::{GameCommand, UiCommand};
use civ_events::GameEvent;
use civ_simulation::SimulationState;

use crate::channel::UiChannel;
use crate::protocol::{IpcEnvelope, IpcMessage, PendingResponses};

pub fn poll_ui_requests(
    channel: Res<UiChannel>,
    registry: Res<FrontApiRegistry>,
    mut ui_commands: EventWriter<UiCommand>,
    mut pending_responses: ResMut<PendingResponses>,
    mut input: ResMut<WebviewInputState>,
    simulation: Option<Res<SimulationState>>,
) {
    let receiver = channel.from_engine.lock().expect("ui channel mutex poisoned");
    let ctx = ApiContext {
        simulation: simulation.as_deref(),
    };

    while let Ok(envelope) = receiver.try_recv() {
        match envelope.message {
            IpcMessage::Request { id, op, body } => {
                match registry.dispatch(&op, body, &ctx) {
                    Ok(outcome) => {
                        if let Some(command) = outcome.ui_command {
                            ui_commands.send(command);
                        }
                        pending_responses.push(id, op, outcome.response);
                    }
                    Err(error) => {
                        pending_responses.push(
                            id,
                            op,
                            serde_json::json!({ "error": error.to_string() }),
                        );
                    }
                }
            }
            IpcMessage::Event { op, body } if op == INPUT_FRAME_OP => {
                if let Ok(frame) = serde_json::from_value::<input_frame::InputFrame>(body) {
                    input.apply_frame(&frame);
                }
            }
            IpcMessage::Event { .. } | IpcMessage::Response { .. } => {}
        }
    }
}

pub fn dispatch_ui_commands(
    mut ui_commands: EventReader<UiCommand>,
    mut game_commands: EventWriter<GameCommand>,
) {
    for command in ui_commands.read() {
        if let UiCommand::Dispatch(game_cmd) = command {
            game_commands.send(game_cmd.clone());
        }
    }
}

pub fn flush_command_responses(
    channel: Res<UiChannel>,
    mut pending_responses: ResMut<PendingResponses>,
) {
    for (id, op, body) in pending_responses.queue.drain(..) {
        channel.send_to_engine(IpcEnvelope::new(
            id,
            IpcMessage::Response {
                id,
                op,
                body,
            },
        ));
    }
}

pub fn forward_game_events_to_ui(
    mut outbox: ResMut<EngineEventOutbox>,
    mut game_events: EventReader<GameEvent>,
) {
    for event in game_events.read() {
        match event {
            GameEvent::TurnStarted { turn, active_player } => {
                turn_changed::emit(
                    &mut outbox,
                    turn_changed::TurnChanged {
                        turn: *turn,
                        active_player: active_player.0,
                    },
                );
            }
            GameEvent::GoldChanged { player, total, .. } => {
                player_gold_changed::emit(
                    &mut outbox,
                    player_gold_changed::PlayerGoldChanged {
                        player: player.0,
                        gold: *total,
                    },
                );
            }
            _ => {}
        }
    }
}

pub fn flush_engine_events(
    channel: Res<UiChannel>,
    mut outbox: ResMut<EngineEventOutbox>,
) {
    let mut seq = 0u64;
    for wire in outbox.drain() {
        let civ_channel::WireMessage::Evt { op, body } = wire else {
            continue;
        };
        channel.send_to_engine(IpcEnvelope::new(
            seq,
            IpcMessage::Event { op, body },
        ));
        seq += 1;
    }
}
