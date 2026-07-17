use bevy::prelude::*;

#[derive(Component)]
pub struct RenderCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        RenderCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(6.0, 12.0, 10.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
    ));
}
