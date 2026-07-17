use serde::{Deserialize, Serialize};

pub const OP: &str = "EndTurn";

#[derive(Debug, Deserialize)]
pub struct EndTurnReq {}

#[derive(Debug, Serialize)]
pub struct EndTurnRes {
    pub ok: bool,
}
