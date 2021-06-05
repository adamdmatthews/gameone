#![warn(clippy::all, clippy::pedantic)]

mod game;
use game::Gameone;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugin(Gameone)
        .add_plugins(DefaultPlugins)
        .run();
}
