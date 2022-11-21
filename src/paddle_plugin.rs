use bevy::prelude::*;

use crate::{get_world_position, CameraFlag, HEIGHT, WIDTH};

const PADDLE_DIMENSIONS: Vec2 = Vec2::new(30., 6.);

#[derive(Component)]
struct Paddle {
    target_position: f32,
}

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    sprite: SpriteBundle,
}

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(paddle_setup)
            .add_system(paddle_movement_system);
    }
}

fn paddle_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PaddleBundle {
        paddle: Paddle {
            target_position: 0.,
        },
        sprite: SpriteBundle {
            texture: asset_server.load("textures/paddle.png"),
            sprite: Sprite {
                custom_size: Some(PADDLE_DIMENSIONS),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., -HEIGHT / 2. + 8., 0.),
                ..default()
            },
            ..default()
        },
    });
}

fn paddle_movement_system(
    time: Res<Time>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<CameraFlag>>,
    mut q_paddle: Query<(&mut Paddle, &mut Transform)>,
) {
    let window = windows.get_primary().unwrap();
    let (_, camera_transform) = q_camera.single();
    let (mut paddle, mut paddle_transform) = q_paddle.single_mut();

    if let Some(raw_position) = window.cursor_position() {
        paddle.target_position = get_world_position(raw_position, window, camera_transform)
            .x
            .clamp(
                -WIDTH / 2. + PADDLE_DIMENSIONS.x / 2. + 1.,
                WIDTH / 2. - PADDLE_DIMENSIONS.x / 2. - 1.,
            );
    };

    let diff = paddle.target_position - paddle_transform.translation.x;
    paddle_transform.translation.x += diff * time.delta_seconds() * 10.;
}
