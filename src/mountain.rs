use bevy::{prelude::*, pbr::wireframe::{WireframePlugin, Wireframe}};

use crate::heightmap::Heightmap;

pub struct MountainPlugin;

impl Plugin for MountainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(WireframePlugin)
            .add_startup_system(insert_plane);
    }
}

fn insert_plane(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let heightmap = Heightmap::new(100.0, 1.0);
    
    commands.spawn(PbrBundle {
        mesh: meshes.add(heightmap.compute_mesh()),
        material: materials.add(Color::PURPLE.into()),
        ..Default::default()
    })
    .insert(Wireframe);
}