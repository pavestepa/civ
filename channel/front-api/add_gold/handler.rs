use civ_commands::{GameCommand, UiCommand};
use civ_common::PlayerId;

use crate::front_api::ApiContext;
use super::model::{AddGoldReq, AddGoldRes};

pub use super::model::{AddGoldReq as Req, AddGoldRes as Res};
pub use super::model::OP;

pub fn handle(req: AddGoldReq, _ctx: &ApiContext<'_>) -> (Option<UiCommand>, AddGoldRes) {
    (
        Some(UiCommand::Dispatch(GameCommand::AddGold {
            player: PlayerId(req.player),
            amount: req.amount,
        })),
        AddGoldRes { ok: true },
    )
}
