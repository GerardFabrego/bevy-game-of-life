use bevy::prelude::*;

use crate::GRID_SIZE;

pub const SPRITE_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Cell(CellState);

enum CellState {
    Alive,
    Dead,
    Empty,
}

#[derive(Resource)]
struct SpriteImages {
    empty_cell: Handle<Image>,
    alive_cell: Handle<Image>,
    dead_cell: Handle<Image>,
}

struct IsSimulationRunning(bool);

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.01)))
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        x as f32 * SPRITE_SIZE,
                        y as f32 * SPRITE_SIZE,
                        0.0,
                    ),
                    texture: asset_server.load("sprites/empty_cell.png"),
                    ..default()
                },
                Cell(CellState::Empty),
            ));
        }
    }

    commands.insert_resource(SpriteImages {
        empty_cell: asset_server.load("sprites/empty_cell.png"),
        alive_cell: asset_server.load("sprites/alive_cell.png"),
        dead_cell: asset_server.load("sprites/dead_cell.png"),
    });
}
