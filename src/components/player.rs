use bevy::{math::{Quat, Vec3}, prelude::Component};


#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct PlayerCamera {
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Component)]
pub struct RotationInterpolation {
    pub target_rotation: Quat,
    pub is_rotating: bool,
}