use bevy::{prelude::*, window::CursorGrabMode};

use crate::{components::player::{Player, PlayerCamera}, resources::cursor::CursorLocked};

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut windows: Query<&mut Window>,
	mut cursor_locked: ResMut<CursorLocked>,
) {
	let mut window = windows.single_mut();

	// 초기화 시 커서 고정 및 숨김
	cursor_locked.0 = true;
	window.cursor.visible = false;
	window.cursor.grab_mode = CursorGrabMode::Locked;

	// 체스판 격자 무늬 바닥 생성 (검정과 흰색 타일을 번갈아 배치)
	let tile_size = 3.0; // 타일 크기 (3x3 크기)
	let board_size = 20; // 총 20x20 타일로 바닥을 채운다

	for i in 0..board_size {
			for j in 0..board_size {
					let x_pos = i as f32 * tile_size - (tile_size * board_size as f32) / 2.0;
					let z_pos = j as f32 * tile_size - (tile_size * board_size as f32) / 2.0;

					let color = if (i + j) % 2 == 0 {
							Color::WHITE // 흰색 타일
					} else {
							Color::BLACK // 검정색 타일
					};

					commands.spawn(PbrBundle {
							mesh: meshes.add(Mesh::from(Cuboid::new(tile_size, 0.1, tile_size))),
							material: materials.add(StandardMaterial {
									base_color: color,
									..default()
							}),
							transform: Transform::from_translation(Vec3::new(x_pos, 0.05, z_pos)),
							..default()
					});
			}
	}

	// 플레이어 상자와 카메라 (FPS 스타일)
	commands.spawn((
			PbrBundle {
					mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
					material: materials.add(StandardMaterial {
							base_color: Color::srgb(0.6, 0.6, 1.0),
							..default()
					}),
					transform: Transform::from_xyz(0.0, 0.5, 0.0),
					..default()
			},
			Player { velocity: Vec3::ZERO },
	)).with_children(|parent| {
			// 카메라를 상자 자식으로 추가하여 상자와 함께 움직이도록 설정
			parent.spawn((
					Camera3dBundle {
							transform: Transform::from_xyz(0.0, 1.1, 2.0),
							..default()
					},
					PlayerCamera { pitch: 0.0, yaw: 0.0 },
			));
	});

	// 빛
	commands.spawn(PointLightBundle {
			point_light: PointLight {
					intensity: 1500.0,
					shadows_enabled: true,
					..default()
			},
			transform: Transform::from_xyz(4.0, 8.0, 4.0),
			..default()
	});
}