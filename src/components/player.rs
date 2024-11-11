use bevy::{math::Vec3, prelude::Component};


#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct PlayerCamera {
    pub pitch: f32,
    pub yaw: f32,
}