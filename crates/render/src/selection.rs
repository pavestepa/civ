use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use civ_channel::WebviewInputState;
use civ_ecs::{CityMarker, HexPosition, Selectable, UnitMarker};
use civ_hex::{HexCoordinate, HexLayout, SELECTION_INNER_RADIUS, SELECTION_OUTER_RADIUS};
use civ_simulation::SimulationState;

use crate::camera::RenderCamera;
use crate::hex_mesh::hex_outline_mesh;

#[derive(Component)]
pub struct SelectionHighlight;

#[derive(Resource, Default)]
pub struct SelectionState {
    pub selected: Option<Entity>,
    pub selected_hex: Option<HexCoordinate>,
    highlight: Option<Entity>,
}

pub fn handle_map_click(
    mut input: ResMut<WebviewInputState>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<RenderCamera>>,
    simulation: Res<SimulationState>,
    selectables: Query<
        (Entity, &HexPosition, Option<&UnitMarker>, Option<&CityMarker>),
        With<Selectable>,
    >,
    mut selection: ResMut<SelectionState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(click) = input.take_click() else {
        return;
    };

    let Ok(window) = window.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let viewport = Vec2::new(
        click.x * window.width(),
        click.y * window.height(),
    );

    let Ok(ray) = camera.viewport_to_world(camera_transform, viewport) else {
        return;
    };

    let Some(hex) = pick_hex_from_ray(ray.origin, ray.direction.as_vec3(), &simulation) else {
        clear_highlight(&mut commands, &mut selection);
        selection.selected = None;
        selection.selected_hex = None;
        return;
    };

    let entity = selectables.iter().find_map(|(entity, pos, unit, city)| {
        if pos.0 != hex {
            return None;
        }
        if unit.is_some() || city.is_some() {
            Some(entity)
        } else {
            None
        }
    });

    clear_highlight(&mut commands, &mut selection);
    selection.selected = entity;
    selection.selected_hex = Some(hex);

    spawn_highlight(
        &mut commands,
        &mut meshes,
        &mut materials,
        hex,
        simulation
            .world
            .tiles
            .get(&hex)
            .map(|tile| tile.elevation.0)
            .unwrap_or(0),
        &mut selection,
    );
}

fn pick_hex_from_ray(
    origin: Vec3,
    direction: Vec3,
    simulation: &SimulationState,
) -> Option<HexCoordinate> {
    let dy = direction.y;
    if dy.abs() < f32::EPSILON {
        return None;
    }

    let t = -origin.y / dy;
    if t < 0.0 {
        return None;
    }

    let hit = origin + direction * t;
    let layout = HexLayout::default();
    let hex = HexCoordinate::from_world_position(hit, layout.size);

    if simulation.world.contains(hex) {
        Some(hex)
    } else {
        None
    }
}

fn clear_highlight(commands: &mut Commands, selection: &mut SelectionState) {
    if let Some(highlight) = selection.highlight.take() {
        commands.entity(highlight).despawn();
    }
}

fn spawn_highlight(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    hex: HexCoordinate,
    elevation: i16,
    selection: &mut SelectionState,
) {
    let layout = HexLayout::default();
    let world = layout.grid_overlay(hex, elevation) + Vec3::Y * 0.04;

    let highlight = commands
        .spawn((
            SelectionHighlight,
            Mesh3d(meshes.add(hex_outline_mesh(
                SELECTION_OUTER_RADIUS,
                SELECTION_INNER_RADIUS,
            ))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 0.85, 0.15, 0.95),
                emissive: LinearRgba::rgb(1.0, 0.75, 0.1),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                cull_mode: None,
                ..default()
            })),
            Transform::from_translation(world),
        ))
        .id();

    selection.highlight = Some(highlight);
}
