use bevy::prelude::*;
use civ_simulation::SimulationState;

use crate::components::{CityEntity, HexPosition, TerrainTile, UnitEntity};
use crate::markers::{CityMarker, Selectable, TerrainMarker, UnitMarker};

pub fn sync_example_entities(
    mut commands: Commands,
    simulation: Res<SimulationState>,
    existing: Query<Entity, Or<(With<TerrainMarker>, With<UnitMarker>, With<CityMarker>)>>,
) {
    if !existing.is_empty() {
        return;
    }

    let layout = civ_hex::HexLayout::default();

    for tile in simulation.world.tiles.values() {
        commands.spawn((
            TerrainMarker,
            TerrainTile {
                terrain: tile.terrain,
                elevation: tile.elevation,
                owner: tile.owner,
            },
            HexPosition(tile.coordinate),
            Transform::from_translation(layout.terrain_surface(tile.coordinate, tile.elevation.0)),
            Visibility::default(),
        ));
    }

    for unit in simulation.world.units.values() {
        let elevation = simulation
            .world
            .tiles
            .get(&unit.coordinate)
            .map(|tile| tile.elevation.0)
            .unwrap_or(0);

        commands.spawn((
            UnitMarker,
            Selectable,
            UnitEntity {
                id: unit.id,
                owner: unit.owner,
                name: unit.name.clone(),
            },
            HexPosition(unit.coordinate),
            Transform::from_translation(layout.unit_base(unit.coordinate, elevation)),
            Visibility::default(),
        ));
    }

    for city in simulation.world.cities.values() {
        let elevation = simulation
            .world
            .tiles
            .get(&city.coordinate)
            .map(|tile| tile.elevation.0)
            .unwrap_or(0);

        commands.spawn((
            CityMarker,
            Selectable,
            CityEntity {
                id: city.id,
                owner: city.owner,
                name: city.name.clone(),
            },
            HexPosition(city.coordinate),
            Transform::from_translation(layout.city_base(city.coordinate, elevation)),
            Visibility::default(),
        ));
    }
}
