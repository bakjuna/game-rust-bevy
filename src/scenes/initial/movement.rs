use bevy::{input::ButtonInput, prelude::*};

use crate::components::player::{Player, PlayerCamera};

pub fn movement(
	keys: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	mut player_query: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
	camera_query: Query<&Transform, With<PlayerCamera>>,
) {
	let mut player_transform = player_query.single_mut();
	let camera_transform = camera_query.single();

	let mut direction = Vec3::ZERO;
	let speed = 5.0;

	// 카메라 방향에 따른 이동 벡터 계산
	let forward: Vec3 = camera_transform.forward().into();
	let right: Vec3 = camera_transform.right().into();

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

	if direction.length_squared() > 0.0 {
			direction = direction.normalize();
	}

	player_transform.translation += direction * speed * time.delta_seconds();
}