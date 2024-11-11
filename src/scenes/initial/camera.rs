use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{components::player::PlayerCamera, resources::cursor::CursorLocked};

pub fn camera(
	mut mouse_motion_events: EventReader<MouseMotion>,
	mut query: Query<(&mut Transform, &mut PlayerCamera)>,
	time: Res<Time>,
	cursor_locked: Res<CursorLocked>,
) {
	if !cursor_locked.0 {
			return;
	}

	let (mut transform, mut camera) = query.single_mut();

	let mut delta_x = 0.0;
	let mut delta_y = 0.0;

	for event in mouse_motion_events.read() {
			delta_x += event.delta.x;
			delta_y += event.delta.y;
	}

	// 감도 조절
	let sensitivity = 0.1;
	camera.yaw -= delta_x * sensitivity * time.delta_seconds();
	camera.pitch -= delta_y * sensitivity * time.delta_seconds();

	// 피치 각도를 제한 (-89도에서 89도 사이로 제한)
	camera.pitch = camera.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

	// 카메라 회전 적용
	transform.rotation = Quat::from_axis_angle(Vec3::Y, camera.yaw) * Quat::from_axis_angle(Vec3::X, camera.pitch);
}