use bevy::{prelude::*, window::WindowResolution};
use camera::CameraPlugin;
use ui::MainMenuPlugin;

mod camera;
mod input;
mod ui;

const GRID_SIZE: i32 = 100;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1024f32, 720f32),
                title: String::from("Game of Life"),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((CameraPlugin, MainMenuPlugin))
        .run();
}
