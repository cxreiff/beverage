use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_debug_text_overlay::screen_print;
use rand::Rng;
use std::time::Duration;

use crate::{HEIGHT, WIDTH};

#[derive(Resource)]
struct YarnTracker {
    count: u8,
    timer: Timer,
}

#[derive(Component, Debug)]
struct Yarn;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct YarnBundle {
    _yarn_flag: Yarn,
    _collider_flag: Collider,
    velocity: Velocity,
    sprite: SpriteBundle,
}

pub struct YarnPlugin;
impl Plugin for YarnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(YarnTracker {
            count: 0,
            timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        })
        .add_system(yarn_spawning_system)
        .add_system(yarn_movement_system)
        .add_system(yarn_wall_system)
        .add_system(yarn_collision_system);
    }
}

fn yarn_spawning_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut tracker: ResMut<YarnTracker>,
) {
    let mut rng = rand::thread_rng();
    tracker.timer.tick(time.delta());
    if tracker.timer.finished() && tracker.count < 3 {
        commands.spawn(YarnBundle {
            _yarn_flag: Yarn,
            _collider_flag: Collider,
            velocity: Velocity(
                Vec2::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5).normalize(),
            ),
            sprite: SpriteBundle {
                texture: asset_server.load("textures/yarn.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10., 10.)),
                    ..default()
                },
                ..default()
            },
        });
        tracker.count += 1;
    }
}

fn yarn_movement_system(
    time: Res<Time>,
    mut q_yarn: Query<(&Velocity, &mut Transform), With<Yarn>>,
) {
    for (velocity, mut transform) in q_yarn.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds() * 10.;
        transform.translation.y += velocity.y * time.delta_seconds() * 10.;
    }
}

fn yarn_wall_system(mut q_yarn: Query<(&mut Velocity, &Transform), With<Yarn>>) {
    for (mut velocity, transform) in q_yarn.iter_mut() {
        if transform.translation.x < (-WIDTH / 2.) && velocity.x < 0. {
            velocity.x *= -1.;
            screen_print!("left wall!");
        }
        if transform.translation.x > (WIDTH / 2.) && velocity.x >= 0. {
            velocity.x *= -1.;
            screen_print!("right wall!");
        }
        if transform.translation.y < (-HEIGHT / 2.) && velocity.y < 0. {
            velocity.y *= -1.;
            screen_print!("bottom wall!");
        }
        if transform.translation.y > (HEIGHT / 2.) && velocity.y >= 0. {
            velocity.y *= -1.;
            screen_print!("top wall!");
        }
    }
}

fn yarn_collision_system(
    q_yarn: Query<(Entity, &Transform, &Sprite), With<Yarn>>,
    q_collider: Query<(Entity, &Transform, &Sprite, Option<&Yarn>), With<Collider>>,
) {
    for (entity_yarn, transform_yarn, sprite_yarn) in q_yarn.iter() {
        for (entity_collider, transform_collider, sprite_collider, is_yarn) in q_collider.iter() {
            if entity_yarn == entity_collider {
                continue;
            }

            if let Some(collision) = collide(
                transform_yarn.translation,
                sprite_yarn
                    .custom_size
                    .or(Some(Vec2::new(10., 10.)))
                    .unwrap()
                    * 0.8,
                transform_collider.translation,
                sprite_collider
                    .custom_size
                    .or(Some(Vec2::new(10., 10.)))
                    .unwrap()
                    * 0.8,
            ) {
                screen_print!(
                    "collision: {:?}, {:?}, {:?}, {:?}",
                    entity_yarn,
                    entity_collider,
                    collision,
                    is_yarn
                );
            }
        }
    }
}
