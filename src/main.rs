#![warn(clippy::all, clippy::pedantic)]

use bevy::log::LogPlugin;
use bevy::prelude::*;

fn hello_world() {
    info!("Hello, world!");
}

fn main() {
    App::build()
        .add_plugin(LogPlugin)
        .add_system(hello_world.system())
        .run();
}
