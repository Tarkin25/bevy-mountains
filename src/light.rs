use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(insert_light);
    }
}

fn insert_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_xyz(50.0, 50.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
