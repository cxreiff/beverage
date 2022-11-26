use bevy::prelude::*;

use crate::{GameState, TextureAssets, HEIGHT, WIDTH};

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Play).with_system(background_setup));
    }
}

fn background_setup(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(SpriteBundle {
        texture: textures.border.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
            ..default()
        },
        ..default()
    });
}
