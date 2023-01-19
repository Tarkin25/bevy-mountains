use std::{ops::{Deref, Add}, sync::Arc};

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use dashmap::DashSet;

use crate::{camera_controller::CameraController, pause::GameState};

use super::ChunksConfig;

pub struct ChunkGridPlugin;

impl Plugin for ChunkGridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkGrid>()
        .register_type::<GridCoordinates>()
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(add_grid_coordinates_to_camera).with_system(update_camera_grid_coordinates));
    }
}

fn add_grid_coordinates_to_camera(query: Query<(Entity, &Transform), (With<CameraController>, Without<GridCoordinates>)>, mut commands: Commands, config: Res<ChunksConfig>) {
    query.for_each(|(entity, transform)| {
        commands.entity(entity).insert(GridCoordinates::from_translation(transform.translation, config.size as i32));
    })
}

fn update_camera_grid_coordinates(mut query: Query<(&mut GridCoordinates, &Transform), With<CameraController>>, config: Res<ChunksConfig>) {
    query.for_each_mut(|(mut coordinates, transform)| {
        *coordinates = GridCoordinates::from_translation(transform.translation, config.size as i32);
    })
}

#[derive(Resource, Default)]
pub struct ChunkGrid {
    chunks: Arc<DashSet<GridCoordinates>>,
}

impl Deref for ChunkGrid {
    type Target = Arc<DashSet<GridCoordinates>>;

    fn deref(&self) -> &Self::Target {
        &self.chunks
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct GridCoordinates(pub IVec2);

impl GridCoordinates {
    pub fn from_translation(Vec3 { x, z, .. }: Vec3, chunk_size: i32) -> Self {
        let x = x as i32;
        let x = x + chunk_size / 2 * x.signum();
        let z = z as i32;
        let z = z + chunk_size / 2 * z.signum();

        Self(IVec2::new(x / chunk_size, z / chunk_size))
    }

    pub fn to_translation(self, chunk_size: i32) -> Vec3 {
        Vec3::new((self.0.x * chunk_size) as f32, 0.0, (self.0.y * chunk_size) as f32)
    }
}

impl Deref for GridCoordinates {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add<IVec2> for GridCoordinates {
    type Output = Self;

    fn add(self, rhs: IVec2) -> Self::Output {
        Self(self.0 + rhs)
    }
}
