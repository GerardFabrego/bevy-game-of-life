use bevy::prelude::*;

const CAMERA_MOVE_SPEED: f32 = 15.0;
const CAMERA_ZOOM_SPEED: f32 = 1.0;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Movement {
    translation_speed: Vec3,
    zoom_speed: f32,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (camera_movement, camera_zoom));
    }
}

fn setup(mut commands: Commands, camera_query: Query<Entity, With<Camera2d>>) {
    let camera = camera_query.get_single().unwrap();

    commands.entity(camera).insert(Movement {
        translation_speed: Vec3::ZERO,
        zoom_speed: 0.0,
    });
}

fn camera_movement(
    mut camera: Query<(&mut Transform, &mut Movement), With<Camera2d>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut move_direction = Vec3::ZERO;

    if input.just_pressed(KeyCode::W) {
        move_direction.y += 1.0;
    }
    if input.just_pressed(KeyCode::S) {
        move_direction.y -= 1.0;
    }
    if input.just_pressed(KeyCode::A) {
        move_direction.x -= 1.0;
    }
    if input.just_pressed(KeyCode::D) {
        move_direction.x += 1.0;
    }

    let move_direction = move_direction.normalize_or_zero();

    let (mut transform, mut movement) = camera.get_single_mut().unwrap();

    movement.translation_speed = (movement.translation_speed + move_direction).clamp(
        Vec3::splat(-CAMERA_MOVE_SPEED),
        Vec3::splat(CAMERA_MOVE_SPEED),
    );

    if input.just_pressed(KeyCode::Space) {
        movement.translation_speed = Vec3::ZERO;
    }

    transform.translation += movement.translation_speed * time.delta_seconds();
}

fn camera_zoom(
    mut camera: Query<(&mut Movement, &mut OrthographicProjection), With<Camera2d>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut zoom_direction = 0.0;

    if input.just_pressed(KeyCode::Q) {
        zoom_direction = 0.01;
    }

    if input.just_pressed(KeyCode::E) {
        zoom_direction = -0.01;
    }

    let (mut movement, mut projection) = camera.get_single_mut().unwrap();

    movement.zoom_speed =
        (movement.zoom_speed + zoom_direction).clamp(-CAMERA_ZOOM_SPEED, CAMERA_ZOOM_SPEED);

    projection.scale =
        (projection.scale + movement.zoom_speed * time.delta_seconds()).clamp(1.0, 6.0);

    if (projection.scale - 1.0).abs() < 0.0001
        || (projection.scale - 6.0).abs() < 0.0001
        || input.just_pressed(KeyCode::Space)
    {
        movement.zoom_speed = 0.0;
    }
}
