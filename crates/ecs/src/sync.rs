use bevy::prelude::*;
use civ_simulation::SimulationState;

use crate::components::{CityEntity, HexPosition, TerrainTile, UnitEntity};
use crate::markers::{CityMarker, TerrainMarker, UnitMarker};
use civ_world::components::Elevation;

pub fn sync_example_entities(
    mut commands: Commands,
    simulation: Res<SimulationState>,
    existing: Query<Entity, Or<(With<TerrainMarker>, With<UnitMarker>, With<CityMarker>)>>,
) {
    if !existing.is_empty() {
        return;
    }

    for tile in simulation.world.tiles.values() {
        commands.spawn((
            TerrainMarker,
            TerrainTile {
                terrain: tile.terrain,
                elevation: tile.elevation,
                owner: tile.owner,
            },
            HexPosition(tile.coordinate),
            Transform::from_translation(tile.coordinate.to_world_position(1.0)),
            bevy::prelude::Visibility::default(),
        ));
    }

    for unit in simulation.world.units.values() {
        commands.spawn((
            UnitMarker,
            UnitEntity {
                id: unit.id,
                owner: unit.owner,
                name: unit.name.clone(),
            },
            HexPosition(unit.coordinate),
            Transform::from_translation(unit.coordinate.to_world_position(1.0) + Vec3::Y * 0.5),
            bevy::prelude::Visibility::default(),
        ));
    }

    for city in simulation.world.cities.values() {
        commands.spawn((
            CityMarker,
            CityEntity {
                id: city.id,
                owner: city.owner,
                name: city.name.clone(),
            },
            HexPosition(city.coordinate),
            Transform::from_translation(city.coordinate.to_world_position(1.0) + Vec3::Y * 0.25),
            bevy::prelude::Visibility::default(),
        ));
    }
}
