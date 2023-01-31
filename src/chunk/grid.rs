use std::{
    ops::{Add, Deref, Sub},
    sync::Arc,
};

use bevy::prelude::*;
use dashmap::DashSet;

use crate::{camera_controller::CameraController, pause::GameState};

use super::ChunksConfig;

pub struct ChunkGridPlugin;

impl Plugin for ChunkGridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkGrid>()
            .register_type::<GridCoordinates>()
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(add_grid_coordinates_to_camera)
                    .with_system(update_camera_grid_coordinates),
            );
    }
}

fn add_grid_coordinates_to_camera(
    query: Query<(Entity, &Transform), (With<CameraController>, Without<GridCoordinates>)>,
    mut commands: Commands,
    config: Res<ChunksConfig>,
) {
    query.for_each(|(entity, transform)| {
        commands
            .entity(entity)
            .insert(GridCoordinates::from_translation(
                transform.translation,
                config.size as i32,
            ));
    })
}

fn update_camera_grid_coordinates(
    mut query: Query<(&mut GridCoordinates, &Transform), With<CameraController>>,
    config: Res<ChunksConfig>,
) {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct GridCoordinates {
    pub x: i32,
    pub z: i32,
}

impl GridCoordinates {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn from_translation(Vec3 { x, z, .. }: Vec3, chunk_size: i32) -> Self {
        let x = x as i32;
        let x = x + chunk_size / 2 * x.signum();
        let z = z as i32;
        let z = z + chunk_size / 2 * z.signum();

        Self {
            x: x / chunk_size,
            z: z / chunk_size,
        }
    }

    pub fn to_translation(self, chunk_size: i32) -> Vec3 {
        Vec3::new(
            (self.x * chunk_size) as f32,
            0.0,
            (self.z * chunk_size) as f32,
        )
    }

    pub fn distance_squared(self, rhs: Self) -> u32 {
        (self - rhs).length_squared()
    }

    pub fn length_squared(self) -> u32 {
        self.x.unsigned_abs().pow(2) + self.z.unsigned_abs().pow(2)
    }

    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    pub fn length(self) -> f32 {
        ((self.x as f32).powi(2) + (self.z as f32).powi(2)).sqrt()
    }
}

impl Add for GridCoordinates {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.z += rhs.z;
        self
    }
}

impl Sub for GridCoordinates {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.z -= rhs.z;
        self
    }
}
