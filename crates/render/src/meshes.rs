use bevy::prelude::*;
use civ_ecs::{
    CityEntity, CityMarker, HexPosition, TerrainMarker, TerrainTile, UnitEntity, UnitMarker,
};
use civ_world::components::TerrainKind;

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
    for (entity, tile, pos) in &terrain {
        let color = terrain_color(tile.terrain);
        let y = tile.elevation.0 as f32 * 0.05;

        commands.entity(entity).insert((
            Mesh3d(meshes.add(Cylinder::new(0.9, 0.2))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                perceptual_roughness: 0.85,
                ..default()
            })),
            Transform::from_translation(pos.0.to_world_position(1.0) + Vec3::Y * y),
        ));
    }

    for (entity, _unit, pos) in &units {
        commands.entity(entity).insert((
            Mesh3d(meshes.add(Capsule3d::new(0.25, 0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.45, 0.85),
                ..default()
            })),
            Transform::from_translation(pos.0.to_world_position(1.0) + Vec3::Y * 0.5),
        ));
    }

    for (entity, _city, pos) in &cities {
        commands.entity(entity).insert((
            Mesh3d(meshes.add(Cuboid::new(0.8, 0.6, 0.8))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.85, 0.65, 0.2),
                ..default()
            })),
            Transform::from_translation(pos.0.to_world_position(1.0) + Vec3::Y * 0.3),
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
