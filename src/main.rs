use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{close_on_esc, CursorGrabMode},
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use camera_controller::CameraControllerPlugin;
use chunk::ChunkPlugin;
use learn_shaders::LearnShadersPlugin;
use light::LightPlugin;
use noise_graph::NoiseGraphPlugin;
use pause::PausePlugin;
use wireframe_controller::WireframeControllerPlugin;

pub mod camera_controller;
pub mod light;
pub mod noise_graph;
pub mod pause;
pub mod wireframe_controller;
pub mod learn_shaders;
pub mod chunk;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Mountains".into(),
                mode: WindowMode::Fullscreen,
                cursor_grab_mode: CursorGrabMode::Locked,
                cursor_visible: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(LightPlugin)
        .add_plugin(CameraControllerPlugin {
            transform: Transform::from_xyz(0.0, 100.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        })
        .add_plugin(LearnShadersPlugin)
        .add_plugin(WireframeControllerPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(ChunkPlugin)
        .add_plugin(NoiseGraphPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(DefaultInspectorConfigPlugin)
        .add_system(close_on_esc)
        .run();
}
