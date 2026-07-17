use civ_common::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub is_human: bool,
    pub gold: i32,
}

impl Player {
    pub fn human(name: impl Into<String>, id: PlayerId) -> Self {
        Self {
            id,
            name: name.into(),
            is_human: true,
            gold: 0,
        }
    }
}
