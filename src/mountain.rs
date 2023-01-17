use bevy::prelude::*;
use noise::{Perlin, NoiseFn, Terrace};

use crate::heightmap::Heightmap;

pub struct MountainPlugin;

impl Plugin for MountainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, insert_noise_config)
            .add_startup_system(insert_plane);
    }
}

fn insert_plane(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, noise_config: Res<NoiseConfig>) {    
    let mut heightmap = Heightmap::new(100.0, 0.5);
    heightmap.compute_heights(|x, z| {
        noise_config.noise.get([x as f64 * 0.1, z as f64 * 0.1]) as f32 * 50.0
    });
    
    commands.spawn(PbrBundle {
        mesh: meshes.add(heightmap.compute_mesh()),
        material: materials.add(Color::PURPLE.into()),
        ..Default::default()
    });
}

#[derive(Resource)]
pub struct NoiseConfig {
    pub noise: Box<dyn NoiseFn<f64, 2> + Send + Sync>,
}

fn insert_noise_config(mut commands: Commands) {
    let perlin = Perlin::default();
    let terrace = Terrace::new(perlin)
    .add_control_point(-1.0)
    .add_control_point(-0.5)
    .add_control_point(0.1)
    .add_control_point(1.0);

    commands.insert_resource(NoiseConfig { noise: Box::new(terrace) });
}