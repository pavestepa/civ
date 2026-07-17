use serde::{Deserialize, Serialize};

pub const OP: &str = "AddGold";

#[derive(Debug, Deserialize)]
pub struct AddGoldReq {
    pub player: u32,
    pub amount: i32,
}

#[derive(Debug, Serialize)]
pub struct AddGoldRes {
    pub ok: bool,
}
