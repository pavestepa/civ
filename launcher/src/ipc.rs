use civ_commands::{GameCommand, UiCommand};
use civ_common::PlayerId;
use civ_hex::HexCoordinate;

use crate::commands::UiAction;

pub fn parse_command(action: UiAction) -> UiCommand {
    match action {
        UiAction::EndTurn => UiCommand::Dispatch(GameCommand::EndTurn),
        UiAction::AddGold { player, amount } => UiCommand::Dispatch(GameCommand::AddGold {
            player: PlayerId(player),
            amount,
        }),
        UiAction::SpawnUnit {
            owner,
            q,
            r,
            kind,
        } => UiCommand::Dispatch(GameCommand::SpawnUnit {
            owner: PlayerId(owner),
            at: HexCoordinate::new(q, r),
            kind,
        }),
        UiAction::CreateCity {
            owner,
            q,
            r,
            name,
        } => UiCommand::Dispatch(GameCommand::CreateCity {
            owner: PlayerId(owner),
            at: HexCoordinate::new(q, r),
            name,
        }),
        UiAction::RequestSnapshot => UiCommand::RequestStateSnapshot,
    }
}
