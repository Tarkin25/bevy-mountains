use std::sync::Arc;

use bevy::prelude::*;
use bevy_inspector_egui::{InspectorOptions, prelude::*, quick::ResourceInspectorPlugin};
use noise::{NoiseFn, Cylinders};

pub struct NoisePlugin;

impl Plugin for NoisePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<GeneralNoiseConfig>()
        .register_type::<GeneralNoiseConfig>()
        .add_plugin(ResourceInspectorPlugin::<GeneralNoiseConfig>::default())
        .add_startup_system_to_stage(StartupStage::PreStartup, insert_terrain_generator);
    }
}

#[derive(Resource, Reflect, InspectorOptions, PartialEq, Clone)]
#[reflect(InspectorOptions)]
pub struct GeneralNoiseConfig {
    pub amplitude: f32,
    pub scale: f64,
}

impl Default for GeneralNoiseConfig {
    fn default() -> Self {
        Self {
            amplitude: 10.0,
            scale: 0.05,
        }
    }
}

#[derive(Clone, Resource)]
pub struct TerrainGenerator(Arc<dyn NoiseFn<f64, 2> + Send + Sync>);

impl TerrainGenerator {
    pub fn compute_height(&self, amplitude: f32, scale: f64, [x, z]: [f32; 2]) -> f32 {
        self.0.get([x as f64 * scale, z as f64 * scale]) as f32 * amplitude
    }
}

fn insert_terrain_generator(mut commands: Commands) {
    let noise = Cylinders::default();
    
    commands.insert_resource(TerrainGenerator(Arc::new(noise)));
}