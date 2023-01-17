use bevy::prelude::*;
use bevy_inspector_egui::{InspectorOptions, prelude::*, quick::ResourceInspectorPlugin};

pub struct NoisePlugin;

impl Plugin for NoisePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<GeneralNoiseConfig>()
        .register_type::<GeneralNoiseConfig>()
        .add_plugin(ResourceInspectorPlugin::<GeneralNoiseConfig>::default());
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