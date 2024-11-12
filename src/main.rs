use crate::resources::cursor::CursorLocked;
use bevy::prelude::*;
use scenes::initial::{
    camera::camera, jump::jump, movement::movement, setup::setup, toggle_cursor::toggle_cursor,
};

mod components;
mod resources;
mod scenes;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .add_systems(Update, jump)
        .add_systems(Update, camera)
        .add_systems(Update, toggle_cursor)
        .insert_resource(CursorLocked(true)) 
        .run();
}
