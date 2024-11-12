use bevy::prelude::*;

use crate::components::player::Player;
pub fn jump(
	keys: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	mut query: Query<(&mut Transform, &mut Player)>,
) {
	let (mut transform, mut player) = query.single_mut();

	// 점프 초기 설정
	let jump_strength = 14.0; // 초기 점프 속도
	let min_gravity = 2.0;   // 땅에 가까울 때 중력
	let max_gravity = 32.0;  // 최고점 중간까지 가는 중력
	let hover_gravity = 8.0; // 최고점에서의 체공 중력

	// 땅에 있을 때 점프 시작
	if keys.just_pressed(KeyCode::Space) && transform.translation.y <= 0.51 {
			player.velocity.y = jump_strength;
	}

	// 현재 높이에 따른 중력 설정
	let height = transform.translation.y;
	let gravity = if height < 1.0 {
			min_gravity + (max_gravity - min_gravity) * (height / 1.0) // 땅에 가까울수록 중력 낮음
	} else if height < 5.0 {
			max_gravity
	} else {
			hover_gravity + (max_gravity - hover_gravity) * ((5.0 - height) / 5.0) // 최고점에서 체공
	};

	// 중력 및 속도 적용
	player.velocity.y -= gravity * time.delta_seconds();
	transform.translation.y += player.velocity.y * time.delta_seconds();

	// 땅에 닿으면 속도 초기화
	if transform.translation.y < 0.5 {
			transform.translation.y = 0.5;
			player.velocity.y = 0.0;
	}
}
