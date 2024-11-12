use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{components::player::{Player, PlayerCamera}, resources::cursor::CursorLocked};

pub fn camera(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut PlayerCamera)>,
    time: Res<Time>,
    cursor_locked: Res<CursorLocked>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    if !cursor_locked.0 {
        return;
    }

    let (mut transform, mut camera) = query.single_mut();

    // Process mouse movement
    let mut delta_x = 0.0;
    let mut delta_y = 0.0;

    for event in mouse_motion_events.read() {
        delta_x += event.delta.x;
        delta_y += event.delta.y;
    }

    // Adjust sensitivity and direction
    let sensitivity = 0.1;
    camera.yaw += delta_x * sensitivity * time.delta_seconds();
    camera.pitch += delta_y * sensitivity * time.delta_seconds();

    // Limit pitch angle (-89 to 89 degrees)
    camera.pitch = camera.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

    // Set camera focus on player position
    let player_transform = player_query.single();
    let focal_point = player_transform.translation + Vec3::new(0.0, 1.1, 0.0);

    let distance = 4.0; // Distance between camera and focus

    // Calculate camera position using spherical coordinates
    let x = focal_point.x + distance * camera.yaw.cos() * camera.pitch.cos();
    let y = focal_point.y + distance * camera.pitch.sin();
    let z = focal_point.z + distance * camera.yaw.sin() * camera.pitch.cos();

    // Update camera position
    transform.translation = Vec3::new(x, y, z);

    // Make camera look at the focal point
    transform.look_at(focal_point, Vec3::Y);
}