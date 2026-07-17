use bevy::prelude::*;

use civ_channel::WebviewInputState;

use crate::camera::RenderCamera;

#[derive(Resource)]
pub struct MapCameraController {
    pub focal_point: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub pan_speed: f32,
    pub zoom_speed: f32,
    pub min_distance: f32,
    pub max_distance: f32,
}

impl Default for MapCameraController {
    fn default() -> Self {
        Self {
            focal_point: Vec3::ZERO,
            distance: 16.0,
            yaw: -0.6,
            pitch: 0.7,
            pan_speed: 12.0,
            zoom_speed: 1.2,
            min_distance: 4.0,
            max_distance: 40.0,
        }
    }
}

pub fn init_map_camera_controller(mut commands: Commands) {
    commands.init_resource::<MapCameraController>();
}

pub fn update_map_camera_controller(
    time: Res<Time>,
    mut input: ResMut<WebviewInputState>,
    mut controller: ResMut<MapCameraController>,
    mut camera: Query<&mut Transform, With<RenderCamera>>,
) {
    let Ok(mut transform) = camera.get_single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let mut pan = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        pan.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        pan.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        pan.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        pan.x += 1.0;
    }

    if input.pressed(KeyCode::KeyQ) {
        controller.distance =
            (controller.distance + controller.zoom_speed * dt * 8.0).min(controller.max_distance);
    }
    if input.pressed(KeyCode::KeyE) {
        controller.distance =
            (controller.distance - controller.zoom_speed * dt * 8.0).max(controller.min_distance);
    }

    let scroll_delta = input.take_scroll_delta();
    if scroll_delta != 0.0 {
        let delta = scroll_delta * 0.05;
        controller.distance = (controller.distance - delta * controller.zoom_speed)
            .clamp(controller.min_distance, controller.max_distance);
    }

    if pan != Vec2::ZERO {
        let rotation = Quat::from_rotation_y(controller.yaw);
        let forward = rotation * -Vec3::Z;
        let right = rotation * Vec3::X;
        let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        let right_xz = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();
        let movement = (forward_xz * pan.y + right_xz * pan.x).normalize_or_zero()
            * controller.pan_speed
            * dt
            * (controller.distance / 16.0);
        controller.focal_point += movement;
    }

    let offset = Quat::from_rotation_y(controller.yaw)
        * Quat::from_rotation_x(-controller.pitch)
        * Vec3::new(0.0, 0.0, controller.distance);

    transform.translation = controller.focal_point + offset;
    transform.look_at(controller.focal_point, Vec3::Y);
}
