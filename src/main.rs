use bevy::{prelude::*, window::WindowResolution};

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
        .run();
}
