use super::config::Config;
use super::world::World;

use bevy::prelude::*;

pub struct Gameone;
impl Plugin for Gameone {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(Config).add_plugin(World);
    }
}
