use bevy::{prelude::*, window::CursorGrabMode};

use crate::resources::cursor::CursorLocked;

pub fn toggle_cursor(
	mut windows: Query<&mut Window>,
	keys: Res<ButtonInput<KeyCode>>,
	mut cursor_locked: ResMut<CursorLocked>,
) {
	if keys.just_pressed(KeyCode::Escape) {
			let mut window = windows.single_mut();
			cursor_locked.0 = !cursor_locked.0;

			// 커서 고정 및 가시성 설정
			window.cursor.visible = !cursor_locked.0;
			window.cursor.grab_mode = if cursor_locked.0 {
					CursorGrabMode::Locked
			} else {
					CursorGrabMode::None
			};
	}
}