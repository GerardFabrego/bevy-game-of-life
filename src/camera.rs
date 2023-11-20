use bevy::prelude::*;

use crate::{simulation::SPRITE_SIZE, GRID_SIZE};

const CAMERA_MOVE_SPEED: f32 = 400.0;
const CAMERA_ZOOM_SPEED: f32 = 5.0;
const MIN_ZOOM: f32 = 1.0;
const MAX_ZOOM: f32 = 5.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (camera_movement, camera_zoom));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            (GRID_SIZE as f32 / 2.0) * SPRITE_SIZE,
            (GRID_SIZE as f32 / 2.0) * SPRITE_SIZE,
            0.0,
        ),
        ..default()
    });
}

fn camera_movement(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut move_direction = Vec3::ZERO;

    if input.pressed(KeyCode::W) {
        move_direction.y += 1.0;
    }
    if input.pressed(KeyCode::S) {
        move_direction.y -= 1.0;
    }
    if input.pressed(KeyCode::A) {
        move_direction.x -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        move_direction.x += 1.0;
    }

    let move_direction = move_direction.normalize_or_zero();

    let mut transform = camera.get_single_mut().unwrap();

    transform.translation += move_direction * CAMERA_MOVE_SPEED * time.delta_seconds();
}

fn camera_zoom(
    mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut zoom_direction = 0.0;

    if input.pressed(KeyCode::Q) {
        zoom_direction = 1.0;
    }

    if input.pressed(KeyCode::E) {
        zoom_direction = -1.0;
    }

    let mut projection = camera.get_single_mut().unwrap();

    projection.scale = (projection.scale
        + (zoom_direction * CAMERA_ZOOM_SPEED * time.delta_seconds()))
    .clamp(MIN_ZOOM, MAX_ZOOM);
}
