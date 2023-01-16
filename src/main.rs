use bevy::{prelude::*, window::{close_on_esc, CursorGrabMode}};
use camera_controller::CameraControllerPlugin;
use light::LightPlugin;

pub mod camera_controller;
pub mod light;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Mountains".into(),
                mode: WindowMode::Fullscreen,
                cursor_grab_mode: CursorGrabMode::Confined,
                cursor_visible: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(LightPlugin)
        .add_plugin(CameraControllerPlugin { transform: Transform::from_xyz(10.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y) })
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
