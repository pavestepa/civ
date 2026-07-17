use bevy::prelude::Resource;
use civ_common::{CivError, CivResult};
use postcard;

use crate::SaveGame;

#[derive(Resource, Default)]
pub struct SaveService;

impl SaveService {
    pub fn serialize(&self, save: &SaveGame) -> CivResult<Vec<u8>> {
        postcard::to_allocvec(save)
            .map_err(|e| CivError::Serialization(e.to_string()))
    }

    pub fn deserialize(&self, bytes: &[u8]) -> CivResult<SaveGame> {
        postcard::from_bytes(bytes).map_err(|e| CivError::Deserialization(e.to_string()))
    }
}
