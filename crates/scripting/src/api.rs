use bevy::prelude::Resource;
use civ_commands::GameCommand;
use civ_common::PlayerId;
use civ_hex::HexCoordinate;

/// Rust-side API surface exposed to QuickJS scripts.
#[derive(Debug, Default, Clone, Resource)]
pub struct ScriptGameApi;

impl ScriptGameApi {
    pub fn spawn_unit(&self, owner: PlayerId, at: HexCoordinate, kind: &str) -> GameCommand {
        GameCommand::SpawnUnit {
            owner,
            at,
            kind: kind.to_string(),
        }
    }

    pub fn create_city(&self, owner: PlayerId, at: HexCoordinate, name: &str) -> GameCommand {
        GameCommand::CreateCity {
            owner,
            at,
            name: name.to_string(),
        }
    }

    pub fn end_turn(&self) -> GameCommand {
        GameCommand::EndTurn
    }

    pub fn add_gold(&self, player: PlayerId, amount: i32) -> GameCommand {
        GameCommand::AddGold { player, amount }
    }

    pub fn get_player(&self, id: PlayerId) -> GameCommand {
        GameCommand::GetPlayer { id }
    }

    pub fn find_city(&self, name: &str) -> GameCommand {
        GameCommand::FindCity {
            name: name.to_string(),
        }
    }
}
