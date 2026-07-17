use civ_commands::{GameCommand, UiCommand};
use civ_common::PlayerId;
use civ_hex::HexCoordinate;

use crate::front_api::ApiContext;
use super::model::{CreateCityReq, CreateCityRes};

pub use super::model::{CreateCityReq as Req, CreateCityRes as Res};
pub use super::model::OP;

pub fn handle(req: CreateCityReq, _ctx: &ApiContext<'_>) -> (Option<UiCommand>, CreateCityRes) {
    (
        Some(UiCommand::Dispatch(GameCommand::CreateCity {
            owner: PlayerId(req.owner),
            at: HexCoordinate::new(req.q, req.r),
            name: req.name,
        })),
        CreateCityRes { ok: true },
    )
}
