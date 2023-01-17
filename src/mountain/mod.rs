use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Terrace};

use crate::{heightmap::Heightmap, pause::GameState};

use self::terrain_noise::{GeneralNoiseConfig, NoisePlugin};

mod terrain_noise;

pub struct MountainPlugin;

impl Plugin for MountainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NoisePlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, insert_noise_config)
            .add_startup_system(insert_plane)
            .add_system_set(SystemSet::on_enter(GameState::Running).with_system(generate_plane));
    }
}

#[derive(Component)]
struct GeneratedPlane;

fn insert_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    noise_generator: Res<NoiseGenerator>,
    noise_config: Res<GeneralNoiseConfig>,
) {
    let mut heightmap = Heightmap::new(100.0, 0.5);
    heightmap.compute_heights(|x, z| {
        noise_generator.noise.get([x as f64 * noise_config.scale, z as f64 * noise_config.scale]) as f32 * noise_config.amplitude
    });

    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(heightmap.compute_mesh()),
            material: materials.add(StandardMaterial {
                base_color: Color::PURPLE,
                metallic: 0.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(heightmap);
}

fn generate_plane(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Heightmap, &Handle<Mesh>)>,
    mut prev_noise_config: Local<GeneralNoiseConfig>,
    noise_config: Res<GeneralNoiseConfig>,
    noise_generator: Res<NoiseGenerator>,
) {
    if *noise_config != *prev_noise_config {
        query.for_each_mut(|(mut heightmap, mesh_handle)| {
            if let Some(mesh) = meshes.get_mut(mesh_handle) {
                heightmap.compute_heights(|x, z| {
                    noise_generator.noise.get([x as f64 * noise_config.scale, z as f64 * noise_config.scale]) as f32 * noise_config.amplitude
                });

                *mesh = heightmap.compute_mesh();
            }
        });

        *prev_noise_config = noise_config.clone();
    }
}

#[derive(Resource)]
pub struct NoiseGenerator {
    pub noise: Box<dyn NoiseFn<f64, 2> + Send + Sync>,
}

fn insert_noise_config(mut commands: Commands) {
    let perlin = Perlin::default();
    let terrace = Terrace::new(perlin)
        .add_control_point(-1.0)
        .add_control_point(-0.5)
        .add_control_point(0.1)
        .add_control_point(1.0);

    commands.insert_resource(NoiseGenerator {
        noise: Box::new(terrace),
    });
}
