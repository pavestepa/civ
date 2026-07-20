use bevy::prelude::*;
use civ_hex::HexLayout;
use civ_simulation::SimulationState;

use crate::components::{HexPosition, TerrainTile};
use crate::markers::{CityMarker, TerrainMarker, UnitMarker};

pub fn snap_entities_to_hex_grid_system(
    simulation: Res<SimulationState>,
    layout: Local<HexLayout>,
    mut terrain: Query<(&HexPosition, &TerrainTile, &mut Transform), With<TerrainMarker>>,
    mut units: Query<(&HexPosition, &mut Transform), (With<UnitMarker>, Without<TerrainMarker>)>,
    mut cities: Query<
        (&HexPosition, &mut Transform),
        (With<CityMarker>, Without<UnitMarker>, Without<TerrainMarker>),
    >,
) {
    let layout = *layout;

    for (hex, tile, mut transform) in &mut terrain {
        *transform = Transform::from_translation(layout.terrain_surface(hex.0, tile.elevation.0));
    }

    for (hex, mut transform) in &mut units {
        let elevation = simulation
            .world
            .tiles
            .get(&hex.0)
            .map(|tile| tile.elevation.0)
            .unwrap_or(0);
        *transform = Transform::from_translation(layout.unit_base(hex.0, elevation));
    }

    for (hex, mut transform) in &mut cities {
        let elevation = simulation
            .world
            .tiles
            .get(&hex.0)
            .map(|tile| tile.elevation.0)
            .unwrap_or(0);
        *transform = Transform::from_translation(layout.city_base(hex.0, elevation));
    }
}
