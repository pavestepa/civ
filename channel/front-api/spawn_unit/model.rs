use serde::{Deserialize, Serialize};

pub const OP: &str = "SpawnUnit";

#[derive(Debug, Deserialize)]
pub struct SpawnUnitReq {
    pub owner: u32,
    pub q: i32,
    pub r: i32,
    pub kind: String,
}

#[derive(Debug, Serialize)]
pub struct SpawnUnitRes {
    pub ok: bool,
}
