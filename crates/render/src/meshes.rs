use bevy::prelude::*;
use civ_ecs::{
    CityEntity, CityMarker, GridCellOutline, HexPosition, TerrainMarker, TerrainTile, UnitEntity,
    UnitMarker,
};
use civ_hex::{TILE_CIRCUMRADIUS, GRID_INNER_RADIUS, GRID_OUTER_RADIUS};
use civ_world::components::TerrainKind;

use crate::hex_mesh::{hex_fill_mesh, hex_outline_mesh};

pub fn attach_example_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain: Query<
        (Entity, &TerrainTile, &HexPosition),
        (With<TerrainMarker>, Without<Mesh3d>),
    >,
    units: Query<(Entity, &UnitEntity, &HexPosition), (With<UnitMarker>, Without<Mesh3d>)>,
    cities: Query<(Entity, &CityEntity, &HexPosition), (With<CityMarker>, Without<Mesh3d>)>,
) {
    let tile_mesh = meshes.add(hex_fill_mesh(TILE_CIRCUMRADIUS));
    let grid_mesh = meshes.add(hex_outline_mesh(GRID_OUTER_RADIUS, GRID_INNER_RADIUS));
    let grid_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.22),
        emissive: LinearRgba::rgb(0.35, 0.45, 0.55),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    for (entity, tile, _pos) in &terrain {
        commands.entity(entity).insert((
            Mesh3d(tile_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: terrain_color(tile.terrain),
                perceptual_roughness: 0.85,
                cull_mode: None,
                ..default()
            })),
        ));

        commands
            .spawn((
                GridCellOutline,
                Mesh3d(grid_mesh.clone()),
                MeshMaterial3d(grid_material.clone()),
                Transform::from_translation(Vec3::Y * 0.04),
            ))
            .set_parent(entity);
    }

    for (entity, _unit, _pos) in &units {
        commands.entity(entity).insert((
            Mesh3d(meshes.add(Capsule3d::new(0.22, 0.45))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.45, 0.85),
                ..default()
            })),
        ));
    }

    for (entity, _city, _pos) in &cities {
        commands.entity(entity).insert((
            Mesh3d(meshes.add(Cuboid::new(0.55, 0.45, 0.55))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.85, 0.65, 0.2),
                ..default()
            })),
        ));
    }
}

fn terrain_color(terrain: TerrainKind) -> Color {
    match terrain {
        TerrainKind::Ocean => Color::srgb(0.1, 0.2, 0.6),
        TerrainKind::Coast => Color::srgb(0.3, 0.5, 0.7),
        TerrainKind::Plains => Color::srgb(0.45, 0.65, 0.3),
        TerrainKind::Grassland => Color::srgb(0.3, 0.7, 0.35),
        TerrainKind::Desert => Color::srgb(0.85, 0.75, 0.45),
        TerrainKind::Tundra => Color::srgb(0.6, 0.65, 0.55),
        TerrainKind::Snow => Color::srgb(0.9, 0.92, 0.95),
        TerrainKind::Hills => Color::srgb(0.4, 0.5, 0.3),
        TerrainKind::Mountain => Color::srgb(0.45, 0.42, 0.4),
    }
}
