use civ_hex::{rect_coordinates, HexCoordinate};

use crate::components::TerrainKind;
use crate::Tile;

pub fn terrain_at(coord: HexCoordinate, seed: u64) -> TerrainKind {
    let hash = coord.q.wrapping_mul(374_761_393)
        ^ coord.r.wrapping_mul(668_265_263)
        ^ seed as i32;
    let bucket = hash.unsigned_abs() % 100;

    match bucket {
        0..=8 => TerrainKind::Ocean,
        9..=14 => TerrainKind::Coast,
        15..=40 => TerrainKind::Plains,
        41..=60 => TerrainKind::Grassland,
        61..=75 => TerrainKind::Hills,
        76..=88 => TerrainKind::Desert,
        _ => TerrainKind::Mountain,
    }
}

pub fn is_land(terrain: TerrainKind) -> bool {
    !matches!(terrain, TerrainKind::Ocean | TerrainKind::Coast)
}

pub fn generate_tiles(width: u32, height: u32, seed: u64) -> indexmap::IndexMap<HexCoordinate, Tile> {
    let mut tiles = indexmap::IndexMap::new();

    for coord in rect_coordinates(width, height) {
        let terrain = terrain_at(coord, seed);
        tiles.insert(
            coord,
            Tile {
                coordinate: coord,
                terrain,
                ..Tile::example_plains(coord)
            },
        );
    }

    tiles
}

pub fn first_land_hex(tiles: &indexmap::IndexMap<HexCoordinate, Tile>) -> Option<HexCoordinate> {
    tiles
        .values()
        .find(|tile| is_land(tile.terrain))
        .map(|tile| tile.coordinate)
}

pub fn land_neighbor(
    tiles: &indexmap::IndexMap<HexCoordinate, Tile>,
    origin: HexCoordinate,
) -> Option<HexCoordinate> {
    origin
        .neighbors()
        .into_iter()
        .find(|coord| tiles.get(coord).is_some_and(|tile| is_land(tile.terrain)))
}
