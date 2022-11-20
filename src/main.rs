// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::App;
use bevy::DefaultPlugins;

use beverage::GamePlugin;
use beverage::ConfigPlugin;

fn main() {
    App::new()
        .add_plugin(ConfigPlugin)    
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
