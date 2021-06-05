use bevy::prelude::*;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource(Msaa { samples: 4 });
    }
}
