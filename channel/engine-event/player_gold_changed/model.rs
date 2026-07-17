pub const OP: &str = "PlayerGoldChanged";

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGoldChanged {
    pub player: u32,
    pub gold: i32,
}
