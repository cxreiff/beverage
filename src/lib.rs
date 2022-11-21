mod config_plugin;
mod kitty_plugin;
mod paddle_plugin;
mod yarn_plugin;

pub use crate::config_plugin::{
    get_world_position, CameraFlag, ConfigPlugin, ASPECT_RATIO, HEIGHT, WIDTH,
};

use crate::kitty_plugin::KittyPlugin;
use crate::paddle_plugin::PaddlePlugin;
use crate::yarn_plugin::YarnPlugin;
use bevy::prelude::{App, Plugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Start,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Start)
            .add_plugin(KittyPlugin)
            .add_plugin(PaddlePlugin)
            .add_plugin(YarnPlugin);
    }
}
