use bevy::prelude::*;
use civ_hex::{HexCoordinate, HexLayout};

#[derive(Component)]
pub struct RenderCamera;

pub fn spawn_camera(mut commands: Commands) {
    let center = HexCoordinate::new(6, 6).to_world_position(HexLayout::default().size);
    commands.spawn((
        RenderCamera,
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.05, 0.08, 0.12)),
            ..default()
        },
        Transform::from_xyz(center.x + 6.0, 12.0, center.z + 10.0).looking_at(center, Vec3::Y),
    ));
}
