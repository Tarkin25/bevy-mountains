use assets::AssetsPlugin;
use bevy::{prelude::*, window::close_on_esc};
use bevy_atmosphere::prelude::*;
use bevy_egui::EguiPlugin;
use camera_controller::{CameraController, CameraControllerPlugin};
use chunk::ChunkPlugin;
use daylight_cycle::DaylightCyclePlugin;
use in_game_time::InGameTimePlugin;
use learn_shaders::LearnShadersPlugin;
use noise_graph::NoiseGraphPlugin;
use pause::PausePlugin;
use velocity::VelocityPlugin;
use wireframe_controller::WireframeControllerPlugin;

pub mod assets;
pub mod camera_controller;
pub mod chunk;
pub mod daylight_cycle;
pub mod in_game_time;
pub mod learn_shaders;
pub mod noise_graph;
pub mod pause;
pub mod velocity;
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
        .add_plugin(CameraControllerPlugin)
        .add_plugin(LearnShadersPlugin)
        .add_plugin(WireframeControllerPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(ChunkPlugin)
        .add_plugin(NoiseGraphPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(VelocityPlugin)
        .add_plugin(InGameTimePlugin)
        .add_plugin(DaylightCyclePlugin)
        .add_system(close_on_esc)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                far: 10_000.0,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..Default::default()
        },
        AtmosphereCamera::default(),
        CameraController::default(),
    ));
}
