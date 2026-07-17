use serde::{Deserialize, Serialize};

pub const OP: &str = "CreateCity";

#[derive(Debug, Deserialize)]
pub struct CreateCityReq {
    pub owner: u32,
    pub q: i32,
    pub r: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateCityRes {
    pub ok: bool,
}
