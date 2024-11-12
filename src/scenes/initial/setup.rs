use std::f32::consts::PI;

use bevy::{prelude::*, window::CursorGrabMode};

use crate::{
    components::player::{Player, PlayerCamera, RotationInterpolation},
    resources::cursor::CursorLocked,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<&mut Window>,
    mut cursor_locked: ResMut<CursorLocked>,
) {
    let mut window = windows.single_mut();

    // Lock and hide cursor on initialization
    cursor_locked.0 = true;
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;

    // Create a chessboard-like floor (alternating black and white tiles)
    let tile_size = 3.0;
    let board_size = 20;

    for i in 0..board_size {
        for j in 0..board_size {
            let x_pos = i as f32 * tile_size - (tile_size * board_size as f32) / 2.0;
            let z_pos = j as f32 * tile_size - (tile_size * board_size as f32) / 2.0;

            let color = if (i + j) % 2 == 0 {
                Color::WHITE
            } else {
                Color::BLACK
            };

            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid::new(tile_size, 0.1, tile_size))),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(x_pos, -0.1, z_pos)),
                ..default()
            });
        }
    }

    // Player box and camera (FPS style)
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid::new(0.0, 0.0, 0.0))),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(55.0, 123.0, 100.0),
                    ..default()
                }),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.5, 0.0),
                    rotation: Quat::from_rotation_y(PI),
                    ..default()
                },
                ..default()
            },
            Player {
                velocity: Vec3::ZERO,
            },
            RotationInterpolation {
                target_rotation: Quat::IDENTITY,
                is_rotating: false,
            },
        ))
        .with_children(|parent| {
            // Front part, red color
            parent.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid::new(0.3, 1.0, 2.0))),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(255.0 / 255.0, 1.0 / 255.0, 1.0 / 255.0),
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(0.25, 0.0, 0.0)),
                ..default()
            });

            // Back part, custom color (55, 123, 100)
            parent.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid::new(0.3, 1.0, 2.0))),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(55.0 / 255.0, 123.0 / 255.0, 100.0 / 255.0),
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(-0.25, 0.0, -0.0)),
                ..default()
            });

            // Attach camera as a child to move with the player
        });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.1, -4.0),
            ..default()
        },
        PlayerCamera {
            pitch: 0.0,
            yaw: 0.0,
        },
    ));

    // Light source
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}