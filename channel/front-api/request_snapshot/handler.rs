use crate::front_api::ApiContext;
use super::model::{RequestSnapshotReq, RequestSnapshotRes};

pub use super::model::{RequestSnapshotReq as Req, RequestSnapshotRes as Res};
pub use super::model::OP;

pub fn handle(
    _req: RequestSnapshotReq,
    ctx: &ApiContext<'_>,
) -> (Option<civ_commands::UiCommand>, RequestSnapshotRes) {
    let snapshot = ctx
        .simulation
        .map(|state| serde_json::to_value(&state.world).unwrap_or(serde_json::Value::Null))
        .unwrap_or(serde_json::Value::Null);

    (
        None,
        RequestSnapshotRes { snapshot },
    )
}
