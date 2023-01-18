use bevy::prelude::*;

use self::{terrain_noise::NoisePlugin, chunk::ChunkPlugin};

mod terrain_noise;
mod mesh;
mod chunk;

pub struct MountainPlugin;

impl Plugin for MountainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NoisePlugin)
            .add_plugin(ChunkPlugin);
    }
}
