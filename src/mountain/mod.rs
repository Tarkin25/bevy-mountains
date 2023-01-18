use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Terrace, Add, Cylinders, Displace, Billow};

use crate::pause::GameState;

use self::{terrain_noise::{GeneralNoiseConfig, NoisePlugin}, mesh::create_mesh};

mod terrain_noise;
mod mesh;

pub struct MountainPlugin;

impl Plugin for MountainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NoisePlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, insert_noise_config)
            .add_startup_system(insert_plane)
            .add_system_set(SystemSet::on_enter(GameState::Running));
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
    let mesh = create_mesh(100.0, 0.5, Vec3::ZERO, |x, z| {
        noise_generator.noise.get([x as f64 * noise_config.scale, z as f64 * noise_config.scale]) as f32 * noise_config.amplitude
    });

    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::PURPLE,
                metallic: 0.0,
                ..Default::default()
            }),
            ..Default::default()
        });
}

#[derive(Resource)]
pub struct NoiseGenerator {
    pub noise: Box<dyn NoiseFn<f64, 2> + Send + Sync>,
}

fn insert_noise_config(mut commands: Commands) {
    /* let perlin = Perlin::default();
    let terrace = Terrace::new(perlin)
        .add_control_point(-1.0)
        .add_control_point(-0.5)
        .add_control_point(0.1)
        .add_control_point(1.0);
    let noise = Add::new(terrace, Cylinders::new()); */

    let noise = Cylinders::new();
    let noise = Billow::<Perlin>::default();

    commands.insert_resource(NoiseGenerator {
        noise: Box::new(noise),
    });
}
