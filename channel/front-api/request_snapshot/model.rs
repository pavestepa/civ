use serde::{Deserialize, Serialize};

pub const OP: &str = "RequestSnapshot";

#[derive(Debug, Deserialize)]
pub struct RequestSnapshotReq {}

#[derive(Debug, Serialize)]
pub struct RequestSnapshotRes {
    pub snapshot: serde_json::Value,
}
