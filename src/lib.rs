mod config_plugin;
mod kitty_plugin;

pub use crate::config_plugin::{
    ConfigPlugin,
    CameraFlag,
    get_world_position,
    ASPECT_RATIO,
    WIDTH,
    HEIGHT
};

use bevy::prelude::{App, Plugin};
use crate::kitty_plugin::KittyPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Start
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Start)
            .add_plugin(KittyPlugin);
    }
}
