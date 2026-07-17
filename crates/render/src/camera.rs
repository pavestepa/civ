use bevy::prelude::*;

#[derive(Component)]
pub struct RenderCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        RenderCamera,
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.05, 0.08, 0.12)),
            ..default()
        },
        Transform::from_xyz(6.0, 12.0, 10.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}
