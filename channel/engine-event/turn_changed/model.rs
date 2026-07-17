pub const OP: &str = "TurnChanged";

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnChanged {
    pub turn: u32,
    pub active_player: u32,
}
