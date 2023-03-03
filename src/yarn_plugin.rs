use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use rand::Rng;
use std::time::Duration;

use crate::{GameState, TextureAssets, HEIGHT, WIDTH};

const YARN_SPEED: f32 = 36.;
const YARN_DIMENSIONS: Vec2 = Vec2::new(10., 10.);
const TUNA_DIMENSIONS: Vec2 = Vec2::new(13., 8.);

#[derive(Resource)]
struct YarnTracker {
    current_count: u8,
    spawn_count: u16,
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
            current_count: 0,
            spawn_count: 0,
            timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        })
        .add_system_set(SystemSet::on_enter(GameState::Play).with_system(yarn_setup))
        .add_system_set(
            SystemSet::on_update(GameState::Play)
                .with_system(yarn_spawning_system)
                .with_system(yarn_movement_system)
                .with_system(yarn_wall_system)
                .with_system(yarn_collision_system),
        );
    }
}

fn yarn_setup(mut commands: Commands, textures: Res<TextureAssets>) {
    for translation in vec![
        Vec3::new(-WIDTH / 3., HEIGHT / 4. - 30., 0.),
        Vec3::new(-WIDTH / 6., HEIGHT / 4. - 30., 0.),
        Vec3::new(0., HEIGHT / 4. - 30., 0.),
        Vec3::new(WIDTH / 6., HEIGHT / 4. - 30., 0.),
        Vec3::new(WIDTH / 3., HEIGHT / 4. - 30., 0.),
        Vec3::new(-WIDTH / 3., HEIGHT / 4. - 20., 0.),
        Vec3::new(-WIDTH / 6., HEIGHT / 4. - 20., 0.),
        Vec3::new(0., HEIGHT / 4. - 20., 0.),
        Vec3::new(WIDTH / 6., HEIGHT / 4. - 20., 0.),
        Vec3::new(WIDTH / 3., HEIGHT / 4. - 20., 0.),
        Vec3::new(-WIDTH / 3., HEIGHT / 4., 0.),
        Vec3::new(-WIDTH / 6., HEIGHT / 4., 0.),
        Vec3::new(0., HEIGHT / 4., 0.),
        Vec3::new(WIDTH / 6., HEIGHT / 4., 0.),
        Vec3::new(WIDTH / 3., HEIGHT / 4., 0.),
        Vec3::new(-WIDTH / 3., HEIGHT / 4. - 10., 0.),
        Vec3::new(-WIDTH / 6., HEIGHT / 4. - 10., 0.),
        Vec3::new(0., HEIGHT / 4. - 10., 0.),
        Vec3::new(WIDTH / 6., HEIGHT / 4. - 10., 0.),
        Vec3::new(WIDTH / 3., HEIGHT / 4. - 10., 0.),
        Vec3::new(-WIDTH / 3., HEIGHT / 4., 0.),
        Vec3::new(-WIDTH / 6., HEIGHT / 4., 0.),
        Vec3::new(0., HEIGHT / 4. - 10., 0.),
        Vec3::new(WIDTH / 6., HEIGHT / 4., 0.),
        Vec3::new(WIDTH / 3., HEIGHT / 4., 0.),
        Vec3::new(-WIDTH / 3., HEIGHT / 4. + 10., 0.),
        Vec3::new(-WIDTH / 6., HEIGHT / 4. + 10., 0.),
        Vec3::new(0., HEIGHT / 4. + 10., 0.),
        Vec3::new(WIDTH / 6., HEIGHT / 4. + 10., 0.),
        Vec3::new(WIDTH / 3., HEIGHT / 4. + 10., 0.),
        Vec3::new(-WIDTH / 3., HEIGHT / 4. + 20., 0.),
        Vec3::new(-WIDTH / 6., HEIGHT / 4. + 20., 0.),
        Vec3::new(0., HEIGHT / 4. + 20., 0.),
        Vec3::new(WIDTH / 6., HEIGHT / 4. + 20., 0.),
        Vec3::new(WIDTH / 3., HEIGHT / 4. + 20., 0.),
        Vec3::new(-WIDTH / 3., HEIGHT / 4. + 30., 0.),
        Vec3::new(-WIDTH / 6., HEIGHT / 4. + 30., 0.),
        Vec3::new(0., HEIGHT / 4. + 30., 0.),
        Vec3::new(WIDTH / 6., HEIGHT / 4. + 30., 0.),
        Vec3::new(WIDTH / 3., HEIGHT / 4. + 30., 0.),
    ]
    .iter()
    {
        commands.spawn(TunaBundle {
            _tuna_flag: Tuna,
            _collider_flag: Collider,
            sprite: SpriteBundle {
                texture: textures.brick.clone(),
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
    textures: Res<TextureAssets>,
    time: Res<Time>,
    mut tracker: ResMut<YarnTracker>,
) {
    tracker.timer.tick(time.delta());
    if tracker.timer.finished() && tracker.current_count < 3 {
        commands.spawn(generate_yarn(textures, tracker.spawn_count));
        tracker.current_count += 1;
        tracker.spawn_count += 1;
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
        if transform.translation.x < (-WIDTH / 2. + YARN_DIMENSIONS.x / 2. + 2.) && velocity.x < 0.
        {
            velocity.x *= -1.;
        }
        if transform.translation.x > (WIDTH / 2. - YARN_DIMENSIONS.x / 2. - 2.) && velocity.x >= 0.
        {
            velocity.x *= -1.;
        }
        if transform.translation.y > (HEIGHT / 2. - YARN_DIMENSIONS.x / 2. - 2.) && velocity.y >= 0.
        {
            velocity.y *= -1.;
        }
        if transform.translation.y < (-HEIGHT / 2. - YARN_DIMENSIONS.x / 2.) && velocity.y < 0. {
            tracker.current_count -= 1;
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
                sprite_yarn.custom_size.unwrap_or(Vec2::new(10., 10.)) * 0.95,
                transform_collider.translation,
                sprite_collider.custom_size.unwrap_or(Vec2::new(10., 10.)) * 0.95,
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

fn generate_yarn(textures: Res<TextureAssets>, spawn_count: u16) -> YarnBundle {
    let mut rng = rand::thread_rng();
    let texture = match spawn_count % 3 {
        0 => &textures.yarn0,
        1 => &textures.yarn1,
        2 => &textures.yarn2,
        _ => unreachable!(),
    }
    .clone();
    YarnBundle {
        _yarn_flag: Yarn,
        _collider_flag: Collider,
        velocity: Velocity(Vec2::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>()).normalize()),
        sprite: SpriteBundle {
            texture,
            sprite: Sprite {
                custom_size: Some(YARN_DIMENSIONS),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., -30., 0.),
                ..default()
            },
            ..default()
        },
    }
}
