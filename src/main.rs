use assets::AssetsPlugin;
use bevy::{prelude::*, window::close_on_esc};
use bevy_egui::EguiPlugin;
use camera_controller::CameraControllerPlugin;
use chunk::ChunkPlugin;
use learn_shaders::LearnShadersPlugin;
use light::LightPlugin;
use noise_graph::NoiseGraphPlugin;
use pause::PausePlugin;
use wireframe_controller::WireframeControllerPlugin;

pub mod assets;
pub mod camera_controller;
pub mod chunk;
pub mod learn_shaders;
pub mod light;
pub mod noise_graph;
pub mod pause;
pub mod widgets;
pub mod wireframe_controller;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "World Generator".into(),
                        mode: WindowMode::BorderlessFullscreen,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_plugin(AssetsPlugin)
        .add_plugin(LightPlugin)
        .add_plugin(CameraControllerPlugin {
            transform: Transform::from_xyz(0.0, 100.0, -10.0),
        })
        .add_plugin(LearnShadersPlugin)
        .add_plugin(WireframeControllerPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(ChunkPlugin)
        .add_plugin(NoiseGraphPlugin)
        .add_plugin(EguiPlugin)
        .add_system(close_on_esc)
        .run();
}
