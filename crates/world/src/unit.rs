use civ_common::{PlayerId, UnitId};
use civ_hex::HexCoordinate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: UnitId,
    pub owner: PlayerId,
    pub coordinate: HexCoordinate,
    pub name: String,
    pub health: u32,
    pub moves_remaining: u32,
}

impl Unit {
    pub fn example(name: impl Into<String>, owner: PlayerId, coordinate: HexCoordinate) -> Self {
        Self {
            id: UnitId::new_v4(),
            owner,
            coordinate,
            name: name.into(),
            health: 100,
            moves_remaining: 2,
        }
    }
}
