use bevy::prelude::*;
use std::time::Duration;

use crate::{get_world_position, CameraFlag, GameState, TextureAssets, HEIGHT};

#[derive(Component)]
struct KittyFlag;

#[derive(Bundle)]
struct KittyBundle {
    _kitty_flag: KittyFlag,
    sprite: SpriteBundle,
}

#[derive(Resource)]
struct KittySpawnerTracker {
    timer: Timer,
    count: u8,
}

pub struct KittyPlugin;

impl Plugin for KittyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Play).with_system(kitty_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Play)
                    .with_system(kitty_spawner)
                    .with_system(kitty_mover),
            );
    }
}

fn kitty_setup(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.insert_resource(KittySpawnerTracker {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        count: 0,
    });
    commands.spawn(KittyBundle {
        _kitty_flag: KittyFlag,
        sprite: SpriteBundle {
            texture: textures.kitty.clone(),
            transform: Transform {
                translation: Vec3::new(0., HEIGHT / 2. - 13., 0.),
                scale: Vec3::new(0.5, 0.5, 0.),
                ..default()
            },
            ..default()
        },
    });
}

fn kitty_spawner(mut tracker: ResMut<KittySpawnerTracker>, time: Res<Time>) {
    tracker.timer.tick(time.delta());

    if tracker.timer.finished() && tracker.count < 1 {
        tracker.count += 1;
    }
}

fn kitty_mover(
    time: Res<Time>,
    windows: Res<Windows>,
    mut q: Query<&mut Transform, With<KittyFlag>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<CameraFlag>>,
) {
    let window = windows.get_primary().unwrap();
    let (_, camera_transform) = q_camera.single();

    let world_position = if let Some(position) = window.cursor_position() {
        get_world_position(position, window, camera_transform)
    } else {
        Vec3::new(0., 0., 0.)
    };

    for mut transform in q.iter_mut() {
        let diff = world_position - transform.translation;
        transform.translation.x += diff.x * time.delta_seconds();
    }
}
