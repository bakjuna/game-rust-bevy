use bevy::prelude::*;

use crate::components::player::Player;

pub fn jump(
	keys: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	mut query: Query<(&mut Transform, &mut Player)>,
) {
	let (mut transform, mut player) = query.single_mut();

	if keys.just_pressed(KeyCode::Space) && transform.translation.y <= 0.51 {
			player.velocity.y = 5.0;
	}

	player.velocity.y -= 9.8 * time.delta_seconds();
	transform.translation.y += player.velocity.y * time.delta_seconds();

	if transform.translation.y < 0.5 {
			transform.translation.y = 0.5;
			player.velocity.y = 0.0;
	}
}