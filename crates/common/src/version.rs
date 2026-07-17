use serde::{Deserialize, Serialize};

pub const SAVE_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Default for GameVersion {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 1,
            patch: 0,
        }
    }
}
