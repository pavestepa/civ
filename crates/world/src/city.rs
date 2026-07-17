use civ_common::{CityId, PlayerId};
use civ_hex::HexCoordinate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub id: CityId,
    pub name: String,
    pub owner: PlayerId,
    pub coordinate: HexCoordinate,
    pub population: u32,
}

impl City {
    pub fn example(name: impl Into<String>, owner: PlayerId, coordinate: HexCoordinate) -> Self {
        Self {
            id: CityId::new_v4(),
            name: name.into(),
            owner,
            coordinate,
            population: 1,
        }
    }
}
