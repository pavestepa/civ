use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum WireMessage {
    Req {
        op: String,
        id: u64,
        body: Value,
    },
    Res {
        op: String,
        id: u64,
        body: Value,
    },
    Evt {
        op: String,
        body: Value,
    },
}

impl WireMessage {
    pub fn req(op: impl Into<String>, id: u64, body: Value) -> Self {
        Self::Req {
            op: op.into(),
            id,
            body,
        }
    }

    pub fn res(op: impl Into<String>, id: u64, body: Value) -> Self {
        Self::Res {
            op: op.into(),
            id,
            body,
        }
    }

    pub fn evt(op: impl Into<String>, body: Value) -> Self {
        Self::Evt {
            op: op.into(),
            body,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }
}
