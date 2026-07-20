use civ_common::PlayerId;
use civ_hex::HexCoordinate;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::generation::{first_land_hex, generate_tiles, land_neighbor};
use crate::{City, Player, Tile, Unit};

pub const DEFAULT_MAP_WIDTH: u32 = 12;
pub const DEFAULT_MAP_HEIGHT: u32 = 12;

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
    pub fn generate(width: u32, height: u32, seed: u64) -> Self {
        let tiles = generate_tiles(width, height, seed);
        let mut map = Self {
            width,
            height,
            tiles,
            turn: 1,
            ..Default::default()
        };

        map.players
            .insert(PlayerId(0), Player::human("Rome", PlayerId(0)));

        let Some(capital) = first_land_hex(&map.tiles) else {
            return map;
        };

        map.cities.insert(
            civ_common::CityId::new_v4(),
            City::example("Rome", PlayerId(0), capital),
        );

        if let Some(unit_hex) = land_neighbor(&map.tiles, capital) {
            map.units.insert(
                civ_common::UnitId::new_v4(),
                Unit::example("Warrior", PlayerId(0), unit_hex),
            );
        }

        map
    }

    pub fn contains(&self, coord: HexCoordinate) -> bool {
        coord.contains_in_rect(self.width, self.height)
    }

    pub fn with_example_entities() -> Self {
        Self::generate(DEFAULT_MAP_WIDTH, DEFAULT_MAP_HEIGHT, 42)
    }
}
