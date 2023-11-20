use std::time::Duration;

use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::{
    ui::{GameExitEvent, SimulationStartEvent, SimulationStopEvent},
    GRID_SIZE,
};

pub const SPRITE_SIZE: f32 = 32.0;

#[derive(Component, Debug)]
pub struct Cell(CellState);

#[derive(Debug)]

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

#[derive(Resource)]
struct WorldPositionDraw(Option<Vec2>);

#[derive(Resource)]
struct WorldPositionErase(Option<Vec2>);

#[derive(Resource)]
struct IsSimulationRunning(bool);

#[derive(Resource)]
struct SimulationTimer(Timer);

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.01)))
            .insert_resource(WorldPositionDraw(None))
            .insert_resource(WorldPositionErase(None))
            .insert_resource(IsSimulationRunning(false))
            .insert_resource(SimulationTimer(Timer::new(
                Duration::from_millis(500),
                TimerMode::Repeating,
            )))
            .add_systems(Startup, setup)
            .add_systems(Update, (set_mouse_world_position, draw_cells))
            .add_systems(
                Update,
                (
                    start_simulation,
                    stop_simulation,
                    run_simulation,
                    run_simulation_timer,
                ),
            )
            .add_systems(Update, exit_game);
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

fn set_mouse_world_position(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Transform, &OrthographicProjection)>,
    mut world_position_draw: ResMut<WorldPositionDraw>,
    mut world_position_erase: ResMut<WorldPositionErase>,
    mouse_input: Res<Input<MouseButton>>,
    is_running: Res<IsSimulationRunning>,
) {
    let window = window_query.get_single().unwrap();

    if !is_running.0 {
        if let Some(pos) = window.cursor_position() {
            let (transform, proj) = camera_query.get_single().unwrap();

            if mouse_input.pressed(MouseButton::Left) {
                let world_position = get_mouse_world_position(pos, transform, window, proj);
                *world_position_draw = WorldPositionDraw(Some(world_position.xy()));
            }

            if mouse_input.pressed(MouseButton::Right) {
                let world_position = get_mouse_world_position(pos, transform, window, proj);
                *world_position_erase = WorldPositionErase(Some(world_position.xy()));
            }
        }
    }
}

fn get_mouse_world_position(
    pos: Vec2,
    camera_transform: &Transform,
    window: &Window,
    proj: &OrthographicProjection,
) -> Vec3 {
    let half_width = (window.width() / 2.0) * proj.scale;
    let half_height = (window.height() / 2.0) * proj.scale;

    let center = camera_transform.translation;
    let left = center.x - half_width;
    let bottom = center.y - half_height;

    Vec3::new(
        left + pos.x * proj.scale,
        bottom + (window.height() - pos.y) * proj.scale,
        0.0,
    )
}

fn draw_cells(
    mut cells: Query<(&mut Cell, &mut Handle<Image>, &Transform)>,
    mut world_position_draw: ResMut<WorldPositionDraw>,
    mut world_position_erase: ResMut<WorldPositionErase>,
    sprite_images: Res<SpriteImages>,
    is_running: Res<IsSimulationRunning>,
) {
    let mouse_draw = world_position_draw.0.take();
    let mouse_erase = world_position_erase.0.take();

    if !is_running.0 && (mouse_draw.is_some() || mouse_erase.is_some()) {
        for (mut cell, mut sprite, transform) in cells.iter_mut() {
            if let Some(pos) = mouse_draw {
                if is_inside_cell(pos, transform.translation.xy(), Vec2::splat(SPRITE_SIZE)) {
                    cell.0 = CellState::Alive;
                    *sprite = sprite_images.alive_cell.clone()
                }
            }
            if let Some(pos) = mouse_erase {
                if is_inside_cell(pos, transform.translation.xy(), Vec2::splat(SPRITE_SIZE)) {
                    cell.0 = CellState::Empty;
                    *sprite = sprite_images.empty_cell.clone()
                }
            }
        }
    }
}

fn is_inside_cell(value: Vec2, center: Vec2, dimensions: Vec2) -> bool {
    value.x >= center.x - dimensions.x / 2.0
        && value.x <= center.x + dimensions.x / 2.0
        && value.y >= center.y - dimensions.y / 2.0
        && value.y <= center.y + dimensions.y / 2.0
}

fn exit_game(mut exit_writer: EventWriter<AppExit>, event_reader: EventReader<GameExitEvent>) {
    if !event_reader.is_empty() {
        exit_writer.send(AppExit);
    }
}

fn start_simulation(
    mut simulation_reader: EventReader<SimulationStartEvent>,
    mut is_simulation_running: ResMut<IsSimulationRunning>,
) {
    if simulation_reader.read().next().is_some() {
        is_simulation_running.0 = true;
    }
}

fn stop_simulation(
    mut simulation_reader: EventReader<SimulationStopEvent>,
    mut is_simulation_running: ResMut<IsSimulationRunning>,
) {
    if simulation_reader.read().next().is_some() {
        is_simulation_running.0 = false;
    }
}

fn run_simulation(
    mut cells: Query<(&mut Cell, &mut Handle<Image>)>,
    is_running: Res<IsSimulationRunning>,
    sprite_images: Res<SpriteImages>,
    simulation_timer: Res<SimulationTimer>,
) {
    if simulation_timer.0.finished() && is_running.0 {
        let life_grid: Vec<_> = cells
            .iter_mut()
            .map(|(cell, _)| match cell.0 {
                CellState::Alive => true,
                CellState::Dead | CellState::Empty => false,
            })
            .collect();

        for (index, (mut cell, mut sprite)) in cells.iter_mut().enumerate() {
            let mut neighbour_count = 0;

            let x = index as i32 % GRID_SIZE;
            let y = index as i32 / GRID_SIZE;

            for xi in (x - 1)..(x + 2) {
                for yi in (y - 1)..(y + 2) {
                    if (xi != yi) && (0..GRID_SIZE).contains(&xi) && (0..GRID_SIZE).contains(&yi) {
                        let one_dimension_index = xi + yi * GRID_SIZE;

                        if life_grid[one_dimension_index as usize] {
                            neighbour_count += 1;
                        }
                    }
                }
            }

            if !(2..=3).contains(&neighbour_count) {
                if let CellState::Alive = cell.0 {
                    cell.0 = CellState::Dead;
                    *sprite = sprite_images.dead_cell.clone();
                }
            }

            if neighbour_count == 3 {
                cell.0 = CellState::Alive;
                *sprite = sprite_images.alive_cell.clone();
            }
        }
    }
}

fn run_simulation_timer(time: Res<Time>, mut match_time: ResMut<SimulationTimer>) {
    match_time.0.tick(time.delta());
}
