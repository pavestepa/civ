use crate::components::{Elevation, Owner, ResourceDeposit, River, Road, TerrainKind, Visibility};
use civ_hex::HexCoordinate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub coordinate: HexCoordinate,
    pub terrain: TerrainKind,
    pub elevation: Elevation,
    pub resource: ResourceDeposit,
    pub river: River,
    pub road: Road,
    pub visibility: Visibility,
    pub owner: Owner,
}

impl Tile {
    pub fn example_plains(coordinate: HexCoordinate) -> Self {
        Self {
            coordinate,
            terrain: TerrainKind::Plains,
            elevation: Elevation(0),
            resource: ResourceDeposit::None,
            river: River { flows: false },
            road: Road { level: 0 },
            visibility: Visibility::VISIBLE,
            owner: Owner::default(),
        }
    }
}
