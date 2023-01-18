use std::{ops::{Deref, Add}, sync::Arc};

use bevy::prelude::*;
use dashmap::DashMap;

pub struct ChunkGridPlugin;

impl Plugin for ChunkGridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkGrid>();
    }
}

#[derive(Resource, Default)]
pub struct ChunkGrid {
    chunks: Arc<DashMap<GridCoordinates, ChunkState>>,
}

pub enum ChunkState {
    ComputingMesh,
    Loaded,
}

impl Deref for ChunkGrid {
    type Target = Arc<DashMap<GridCoordinates, ChunkState>>;

    fn deref(&self) -> &Self::Target {
        &self.chunks
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component)]
pub struct GridCoordinates(IVec2);

impl GridCoordinates {
    pub fn from_translation(Vec3 { x, z, .. }: Vec3, chunk_size: i32) -> Self {
        let x = x.round() as i32;
        let z = z.round() as i32;
        Self(IVec2::new(x - x % chunk_size, z - z % chunk_size))
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

impl From<GridCoordinates> for Vec3 {
    fn from(value: GridCoordinates) -> Self {
        Vec3::new(value.0.x as f32, 0.0, value.0.y as f32)
    }
}
