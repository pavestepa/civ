use civ_commands::{GameCommand, UiCommand};

use crate::front_api::ApiContext;
use super::model::{EndTurnReq, EndTurnRes};

pub use super::model::{EndTurnReq as Req, EndTurnRes as Res};
pub use super::model::OP;

pub fn handle(_req: EndTurnReq, _ctx: &ApiContext<'_>) -> (Option<UiCommand>, EndTurnRes) {
    (
        Some(UiCommand::Dispatch(GameCommand::EndTurn)),
        EndTurnRes { ok: true },
    )
}
