use bevy::{prelude::*, window::{close_on_esc, CursorGrabMode}, diagnostic::FrameTimeDiagnosticsPlugin};
use camera_controller::CameraControllerPlugin;
use light::LightPlugin;
use mountain::MountainPlugin;
use pause::PausePlugin;
use wireframe_controller::WireframeControllerPlugin;

pub mod camera_controller;
pub mod light;
pub mod mountain;
pub mod heightmap;
pub mod wireframe_controller;
pub mod pause;

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
        .add_plugin(CameraControllerPlugin { transform: Transform::from_xyz(0.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y) })
        .add_plugin(MountainPlugin)
        .add_plugin(WireframeControllerPlugin)
        .add_plugin(PausePlugin)
        .add_startup_system(insert_cube)
        .add_system(close_on_esc)
        .run();
}

fn insert_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::PURPLE.into()),
        ..Default::default()
    });
}
