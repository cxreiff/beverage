use std::time::Duration;
use bevy::prelude::*;

use crate::{CameraFlag, get_world_position};

pub struct KittyPlugin;

impl Plugin for KittyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(kitty_setup);
        app.add_system(kitty_spawner);
        app.add_system(kitty_mover);
    }
}

#[derive(Component)]
struct KittyFlag;

#[derive(Bundle)]
struct KittyBundle {
    _kitty_flag: KittyFlag,

    #[bundle]
    sprite: SpriteBundle,
}

fn kitty_setup(mut commands: Commands) {
    commands.insert_resource(KittySpawnerTracker {
        timer: Timer::new(Duration::from_secs(3), true),
        count: 0,
    });
}

struct KittySpawnerTracker {
    timer: Timer,
    count: u8,
}

fn kitty_spawner(
    mut commands: Commands,
    mut tracker: ResMut<KittySpawnerTracker>,
    time: Res<Time>,
    asset_server: Res<AssetServer>
) {
    tracker.timer.tick(time.delta());

    if tracker.timer.finished() && tracker.count < 1 {
        commands.spawn_bundle(KittyBundle {
            _kitty_flag: KittyFlag,
            sprite: SpriteBundle {
                texture: asset_server.load("textures/kitty.png"),
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    scale: Vec3::new(1., 1., 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
        });
        tracker.count += 1;
    }
}

fn kitty_mover(
    time: Res<Time>,
    windows: Res<Windows>,
    mut q: Query<&mut Transform, With<KittyFlag>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<CameraFlag>>
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
        transform.translation += diff * time.delta_seconds() * 10.0;
    }
}
