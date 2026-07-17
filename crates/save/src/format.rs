use civ_common::{GameVersion, SAVE_FORMAT_VERSION};
use civ_world::WorldMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    pub format_version: u32,
    pub game_version: GameVersion,
    pub world: WorldMap,
    pub checksum: u64,
}

impl SaveGame {
    pub fn from_world(world: WorldMap) -> Self {
        Self {
            format_version: SAVE_FORMAT_VERSION,
            game_version: GameVersion::default(),
            checksum: 0,
            world,
        }
    }
}
