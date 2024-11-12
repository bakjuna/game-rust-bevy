use bevy::prelude::*;

use crate::components::player::{Player, PlayerCamera, RotationInterpolation};

pub fn movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut RotationInterpolation), (With<Player>, Without<PlayerCamera>)>,
    camera_query: Query<&Transform, With<PlayerCamera>>,
) {
    let (mut player_transform, mut rotation_interpolation) = player_query.single_mut();
    let camera_transform = camera_query.single();

    let mut direction = Vec3::ZERO;
    let speed = 7.0;
    let angular_speed = 5.0; // Rotation speed (rad/s)

    // Get forward and right vectors from the camera (Y-axis set to 0)
    let forward = Vec3::new(camera_transform.forward().x, 0.0, camera_transform.forward().z).normalize();
    let right = Vec3::new(camera_transform.right().x, 0.0, camera_transform.right().z).normalize();

    // Calculate movement direction
    if keys.pressed(KeyCode::KeyW) {
        direction += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= forward;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= right;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += right;
    }

    // Move and rotate if there's a direction
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        player_transform.translation += direction * speed * time.delta_seconds();

        // Calculate target rotation
        let target_yaw = direction.z.atan2(direction.x);
        let target_rotation = Quat::from_rotation_y(-target_yaw);

        // Start rotation if not already rotating
        if !rotation_interpolation.is_rotating || rotation_interpolation.target_rotation != target_rotation {
            rotation_interpolation.target_rotation = target_rotation;
            rotation_interpolation.is_rotating = true;
        }
    }

    // Handle rotation
    if rotation_interpolation.is_rotating && keys.pressed(KeyCode::KeyW) {
        let current_rotation = player_transform.rotation;
        let target_rotation = rotation_interpolation.target_rotation;

        // Calculate rotation angle difference
        let angle = current_rotation.angle_between(target_rotation);

        // Calculate angle to rotate for this frame
        let angle_to_rotate = angular_speed * time.delta_seconds();

        if angle <= angle_to_rotate {
            // Reach target rotation this frame
            player_transform.rotation = target_rotation;
            rotation_interpolation.is_rotating = false;
        } else {
            // Rotate partially towards target
            let t = angle_to_rotate / angle;
            player_transform.rotation = current_rotation.slerp(target_rotation, t);
        }
    }
}