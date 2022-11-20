// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::App;
use bevy::DefaultPlugins;

use beverage::ConfigPlugin;
use beverage::GamePlugin;

fn main() {
    App::new()
        .add_plugin(ConfigPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
