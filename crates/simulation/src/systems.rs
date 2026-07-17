use bevy::prelude::*;
use civ_commands::GameCommand;
use civ_common::{PlayerId, UnitId};
use civ_events::GameEvent;
use civ_world::{City, Unit};

use crate::state::SimulationState;

pub fn process_commands(
    mut commands: EventReader<GameCommand>,
    mut events: EventWriter<GameEvent>,
    mut state: ResMut<SimulationState>,
) {
    for command in commands.read() {
        match command {
            GameCommand::EndTurn => {
                state.world.turn += 1;
                events.send(GameEvent::TurnEnded {
                    turn: state.world.turn,
                });
                events.send(GameEvent::TurnStarted {
                    turn: state.world.turn,
                    active_player: PlayerId(0),
                });
            }
            GameCommand::SpawnUnit { owner, at, kind } => {
                let unit = Unit {
                    id: UnitId::new_v4(),
                    owner: *owner,
                    coordinate: *at,
                    name: kind.clone(),
                    health: 100,
                    moves_remaining: 2,
                };
                let id = unit.id;
                state.world.units.insert(id, unit);
                events.send(GameEvent::UnitSpawned {
                    id,
                    owner: *owner,
                    at: *at,
                });
            }
            GameCommand::CreateCity { owner, at, name } => {
                let city = City::example(name.clone(), *owner, *at);
                let id = city.id;
                state.world.cities.insert(id, city);
                events.send(GameEvent::CityFounded {
                    id,
                    owner: *owner,
                    at: *at,
                });
            }
            GameCommand::AddGold { player, amount } => {
                if let Some(p) = state.world.players.get_mut(player) {
                    p.gold += amount;
                    events.send(GameEvent::GoldChanged {
                        player: *player,
                        delta: *amount,
                        total: p.gold,
                    });
                }
            }
            GameCommand::MoveUnit { unit, to } => {
                if let Some(u) = state.world.units.get_mut(unit) {
                    u.coordinate = *to;
                    events.send(GameEvent::TileChanged { at: *to });
                }
            }
            GameCommand::SelectEntity { .. }
            | GameCommand::FindCity { .. }
            | GameCommand::GetPlayer { .. } => {}
        }
    }
}
