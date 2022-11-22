use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_debug_text_overlay::screen_print;
use rand::Rng;
use std::time::Duration;

use crate::{HEIGHT, WIDTH};

const YARN_SPEED: f32 = 20.;
const YARN_DIMENSIONS: Vec2 = Vec2::new(10., 10.);
const TUNA_DIMENSIONS: Vec2 = Vec2::new(20., 10.);

#[derive(Resource)]
struct YarnTracker {
    count: u8,
    timer: Timer,
}

#[derive(Component, Debug)]
struct Yarn;

#[derive(Component, Debug)]
struct Tuna;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Bundle)]
struct YarnBundle {
    _yarn_flag: Yarn,
    _collider_flag: Collider,
    velocity: Velocity,
    sprite: SpriteBundle,
}

#[derive(Bundle)]
struct TunaBundle {
    _tuna_flag: Tuna,
    _collider_flag: Collider,
    sprite: SpriteBundle,
}

pub struct YarnPlugin;
impl Plugin for YarnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(YarnTracker {
            count: 0,
            timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        })
        .add_startup_system(yarn_setup)
        .add_system(yarn_spawning_system)
        .add_system(yarn_movement_system)
        .add_system(yarn_wall_system)
        .add_system(yarn_collision_system);
    }
}

fn yarn_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    for translation in vec![
        Vec3::new(0., HEIGHT / 4., 0.),
        Vec3::new(-WIDTH / 4., HEIGHT / 4., 0.),
        Vec3::new(WIDTH / 4., HEIGHT / 4., 0.),
        Vec3::new(0., HEIGHT / 4. - 10., 0.),
        Vec3::new(-WIDTH / 4., HEIGHT / 4. - 10., 0.),
        Vec3::new(WIDTH / 4., HEIGHT / 4. - 10., 0.),
    ]
    .iter()
    {
        commands.spawn(TunaBundle {
            _tuna_flag: Tuna,
            _collider_flag: Collider,
            sprite: SpriteBundle {
                texture: asset_server.load("textures/tuna.png"),
                sprite: Sprite {
                    custom_size: Some(TUNA_DIMENSIONS),
                    ..default()
                },
                transform: Transform {
                    translation: *translation,
                    ..default()
                },
                ..default()
            },
        });
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
                    custom_size: Some(YARN_DIMENSIONS),
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
        transform.translation.x += velocity.x * time.delta_seconds() * YARN_SPEED;
        transform.translation.y += velocity.y * time.delta_seconds() * YARN_SPEED;
    }
}

fn yarn_wall_system(
    mut commands: Commands,
    mut tracker: ResMut<YarnTracker>,
    mut q_yarn: Query<(Entity, &mut Velocity, &Transform), With<Yarn>>,
) {
    for (entity, mut velocity, transform) in q_yarn.iter_mut() {
        if transform.translation.x < (-WIDTH / 2. + YARN_DIMENSIONS.x / 2.) && velocity.x < 0. {
            velocity.x *= -1.;
            screen_print!("left wall!");
        }
        if transform.translation.x > (WIDTH / 2. - YARN_DIMENSIONS.x / 2.) && velocity.x >= 0. {
            velocity.x *= -1.;
            screen_print!("right wall!");
        }
        if transform.translation.y > (HEIGHT / 2. - YARN_DIMENSIONS.x / 2.) && velocity.y >= 0. {
            velocity.y *= -1.;
            screen_print!("top wall!");
        }
        if transform.translation.y < (-HEIGHT / 2. + YARN_DIMENSIONS.x / 2.) && velocity.y < 0. {
            tracker.count -= 1;
            commands.entity(entity).despawn();
        }
    }
}

fn yarn_collision_system(
    mut commands: Commands,
    mut q_yarn: Query<(&Transform, &Sprite, &mut Velocity), With<Yarn>>,
    q_collider: Query<
        (Entity, &Transform, &Sprite, Option<&Tuna>),
        (With<Collider>, Without<Yarn>),
    >,
) {
    for (transform_yarn, sprite_yarn, mut velocity) in q_yarn.iter_mut() {
        for (entity_collider, transform_collider, sprite_collider, is_tuna) in q_collider.iter() {
            if let Some(collision) = collide(
                transform_yarn.translation,
                sprite_yarn
                    .custom_size
                    .or(Some(Vec2::new(10., 10.)))
                    .unwrap()
                    * 0.95,
                transform_collider.translation,
                sprite_collider
                    .custom_size
                    .or(Some(Vec2::new(10., 10.)))
                    .unwrap()
                    * 0.95,
            ) {
                if is_tuna.is_some() {
                    commands.entity(entity_collider).despawn();
                }

                match collision {
                    Collision::Left if velocity.x > 0. => velocity.x *= -1.,
                    Collision::Right if velocity.x <= 0. => velocity.x *= -1.,
                    Collision::Bottom if velocity.y > 0. => velocity.y *= -1.,
                    Collision::Top if velocity.y <= 0. => velocity.y *= -1.,
                    _ => (),
                }
            }
        }
    }
}
