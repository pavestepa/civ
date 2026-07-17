use civ_common::PlayerId;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{City, Player, Tile, Unit};
use civ_hex::HexCoordinate;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldMap {
    pub width: u32,
    pub height: u32,
    pub tiles: IndexMap<HexCoordinate, Tile>,
    pub players: IndexMap<PlayerId, Player>,
    pub units: IndexMap<civ_common::UnitId, Unit>,
    pub cities: IndexMap<civ_common::CityId, City>,
    pub turn: u32,
}

impl WorldMap {
    pub fn with_example_entities() -> Self {
        let mut map = Self {
            width: 8,
            height: 8,
            turn: 1,
            ..Default::default()
        };

        let origin = HexCoordinate::ZERO;
        map.tiles.insert(origin, Tile::example_plains(origin));
        map.players
            .insert(PlayerId(0), Player::human("Rome", PlayerId(0)));
        map.units.insert(
            civ_common::UnitId::new_v4(),
            Unit::example("Warrior", PlayerId(0), HexCoordinate::new(1, 0)),
        );
        map.cities.insert(
            civ_common::CityId::new_v4(),
            City::example("Rome", PlayerId(0), origin),
        );

        map
    }
}
