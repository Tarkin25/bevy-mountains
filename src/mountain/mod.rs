use bevy::prelude::*;

use self::chunk::ChunkPlugin;

mod chunk;
mod terrain_noise;

pub struct MountainPlugin;

impl Plugin for MountainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ChunkPlugin);
    }
}
