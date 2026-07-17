use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use civ_channel::WebviewInputState;
use civ_ecs::{CityMarker, HexPosition, Selectable, UnitMarker};
use civ_hex::HexCoordinate;

use crate::camera::RenderCamera;
use crate::hex_mesh::hex_outline_mesh;

const HEX_SIZE: f32 = 1.0;
const HIGHLIGHT_Y: f32 = 0.12;
const OUTER_RADIUS: f32 = 0.94;
const INNER_RADIUS: f32 = 0.82;

#[derive(Component)]
pub struct SelectionHighlight;

#[derive(Resource, Default)]
pub struct SelectionState {
    pub selected: Option<Entity>,
    highlight: Option<Entity>,
}

pub fn handle_map_click(
    mut input: ResMut<WebviewInputState>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<RenderCamera>>,
    selectables: Query<
        (
            Entity,
            &GlobalTransform,
            &HexPosition,
            Option<&UnitMarker>,
            Option<&CityMarker>,
        ),
        (With<Selectable>, With<Mesh3d>),
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

    let ray_dir = ray.direction.as_vec3();
    let mut closest: Option<(Entity, f32, HexCoordinate)> = None;

    for (entity, transform, hex_pos, unit, city) in &selectables {
        if unit.is_none() && city.is_none() {
            continue;
        }

        let radius = if unit.is_some() { 0.5 } else { 0.6 };
        let Some(distance) = ray_sphere_hit(ray.origin, ray_dir, transform.translation(), radius)
        else {
            continue;
        };

        if closest.is_none_or(|(_, best, _)| distance < best) {
            closest = Some((entity, distance, hex_pos.0));
        }
    }

    clear_highlight(&mut commands, &mut selection);

    let Some((entity, _, hex)) = closest else {
        selection.selected = None;
        return;
    };

    selection.selected = Some(entity);
    spawn_highlight(
        &mut commands,
        &mut meshes,
        &mut materials,
        hex,
        &mut selection,
    );
}

fn ray_sphere_hit(origin: Vec3, direction: Vec3, center: Vec3, radius: f32) -> Option<f32> {
    let oc = origin - center;
    let a = direction.dot(direction);
    let b = 2.0 * oc.dot(direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let t1 = (-b - sqrt_d) / (2.0 * a);
    if t1 >= 0.0 {
        return Some(t1);
    }

    let t2 = (-b + sqrt_d) / (2.0 * a);
    (t2 >= 0.0).then_some(t2)
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
    hex: civ_hex::HexCoordinate,
    selection: &mut SelectionState,
) {
    let world = hex.to_world_position(HEX_SIZE) + Vec3::Y * HIGHLIGHT_Y;

    let highlight = commands
        .spawn((
            SelectionHighlight,
            Mesh3d(meshes.add(hex_outline_mesh(OUTER_RADIUS, INNER_RADIUS))),
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
