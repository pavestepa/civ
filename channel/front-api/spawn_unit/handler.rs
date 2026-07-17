use civ_commands::{GameCommand, UiCommand};
use civ_common::PlayerId;
use civ_hex::HexCoordinate;

use crate::front_api::ApiContext;
use super::model::{SpawnUnitReq, SpawnUnitRes};

pub use super::model::{SpawnUnitReq as Req, SpawnUnitRes as Res};
pub use super::model::OP;

pub fn handle(req: SpawnUnitReq, _ctx: &ApiContext<'_>) -> (Option<UiCommand>, SpawnUnitRes) {
    (
        Some(UiCommand::Dispatch(GameCommand::SpawnUnit {
            owner: PlayerId(req.owner),
            at: HexCoordinate::new(req.q, req.r),
            kind: req.kind,
        })),
        SpawnUnitRes { ok: true },
    )
}
