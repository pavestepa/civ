use bevy::prelude::*;

pub fn spawn_lighting(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, -0.5, 0.8, 0.0)),
        ..default()
    });
}
