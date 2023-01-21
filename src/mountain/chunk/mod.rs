use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    utils::HashMap,
};
use bevy_inspector_egui::prelude::*;
use futures_lite::future;
use noise::NoiseFn;

use crate::{camera_controller::CameraController, pause::GameState, noise_graph::NoiseGraph};

use self::grid::{ChunkGrid, ChunkGridPlugin, GridCoordinates};

use super::mesh::create_mesh;

mod grid;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunksConfig>()
            .register_type::<ChunksConfig>()
            .add_plugin(ChunkGridPlugin)
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(trigger_chunk_creation.before(spawn_compute_mesh_tasks))
                    .with_system(update_level_of_detail.before(spawn_compute_mesh_tasks))
                    .with_system(spawn_compute_mesh_tasks.before(insert_mesh))
                    .with_system(insert_mesh.before(unload_chunks))
                    .with_system(unload_chunks)
                    //.with_system(add_center_point_to_chunks)
            )
            .add_system_set(SystemSet::on_enter(GameState::Running).with_system(reload_chunks));
    }
}

fn trigger_chunk_creation(
    query: Query<&GridCoordinates, With<CameraController>>,
    chunk_grid: Res<ChunkGrid>,
    config: Res<ChunksConfig>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if config.load_chunks {
        let render_distance = config.render_distance;

        if let Ok(camera_coordinates) = query.get_single() {
            let mut load_chunk = move |x, z| {
                let chunk_coordinates = *camera_coordinates + GridCoordinates::new(x, z);
    
                if chunk_coordinates.distance_squared(*camera_coordinates) <= render_distance.pow(2) && !chunk_grid.contains(&chunk_coordinates) {
                    chunk_grid.insert(chunk_coordinates);
                    commands.spawn((
                        chunk_coordinates,
                        Chunk {
                            cell_size: config.get_cell_size(chunk_coordinates, *camera_coordinates),
                        },
                        LoadChunk,
                        Transform::from_translation(chunk_coordinates.to_translation(config.size as i32)),
                        GlobalTransform::default(),
                        VisibilityBundle {
                            visibility: Visibility::VISIBLE,
                            computed: ComputedVisibility::default(),
                        },
                        materials.add(StandardMaterial {
                            base_color: Color::PURPLE.into(),
                            metallic: 0.0,
                            reflectance: 0.0,
                            ..Default::default()
                        }),
                    ));
                }
            };
    
            for x in (0..=render_distance as i32).rev() {
                for z in (0..=render_distance as i32).rev() {
                    load_chunk(x, z);
                    load_chunk(-x, z);
                    load_chunk(x, -z);
                    load_chunk(-x, -z);
                }
            }
        }
    }
}

fn spawn_compute_mesh_tasks(
    noise_graph: Res<NoiseGraph>,
    mut commands: Commands,
    query: Query<(Entity, &GridCoordinates, &Chunk), With<LoadChunk>>,
    chunks_config: Res<ChunksConfig>,
) {
    let pool = AsyncComputeTaskPool::get();

    for (entity, grid_coordinates, chunk) in query.iter().take(chunks_config.updates_per_frame) {
        let grid_coordinates = *grid_coordinates;
        let Chunk { cell_size } = *chunk;
        let noise = noise_graph.get_noise_fn().clone();
        let translation = grid_coordinates.to_translation(chunks_config.size as i32);
        let size = chunks_config.size;
        let task = pool.spawn(async move {
            create_mesh(size, cell_size, translation, |x, z| {
                noise.get([x as f64, z as f64]) as f32
            })
        });
        let mut entity = commands.entity(entity);
        entity
            .insert(ComputeMesh(task))
            .remove::<LoadChunk>();
    }
}

fn insert_mesh(
    mut query: Query<(Entity, &mut ComputeMesh)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<ChunksConfig>,
) {
    for (entity, mut task) in query.iter_mut().take(config.updates_per_frame) {
        if let Some(mesh) = future::block_on(future::poll_once(&mut task.0)) {
            let mut entity = commands.entity(entity);
            entity.remove::<ComputeMesh>();
            entity.remove::<Handle<Mesh>>();
            entity.insert(meshes.add(mesh));
        }
    }
}

fn update_level_of_detail(
    mut chunks: Query<(Entity, &mut Chunk, &GridCoordinates)>,
    mut commands: Commands,
    camera: Query<&GridCoordinates, With<CameraController>>,
    config: Res<ChunksConfig>,
) {
    if let Ok(camera_coordinates) = camera.get_single() {
        for (entity, mut chunk, coordinates) in chunks.iter_mut()/* .take(config.updates_per_frame) */ {
            let new_cell_size = config.get_cell_size(*coordinates, *camera_coordinates);
    
            if chunk.cell_size != new_cell_size {
                chunk.cell_size = new_cell_size;
                commands.entity(entity).insert(LoadChunk);
            }
        }
    }
}

fn reload_chunks(mut query: Query<Entity, (Without<LoadChunk>, With<Chunk>)>, mut commands: Commands) {
    query.for_each_mut(|entity| {
        commands.entity(entity).insert(LoadChunk);
    })
}

fn unload_chunks(
    mut commands: Commands,
    chunks: Query<(Entity, &GridCoordinates), (With<Chunk>, Without<LoadChunk>, Without<ComputeMesh>)>,
    camera: Query<&GridCoordinates, With<CameraController>>,
    chunks_config: Res<ChunksConfig>,
    chunk_grid: Res<ChunkGrid>,
) {
    if chunks_config.load_chunks {
        if let Ok(camera) = camera.get_single() {
            chunks.for_each(|(entity, coordinates)| {
                let is_outside_render_distance = camera.distance_squared(*coordinates) > chunks_config.render_distance.pow(2);

                if is_outside_render_distance {
                    commands.entity(entity).despawn_recursive();
                    chunk_grid.remove(coordinates);
                }
            });
        }
    }
}

#[derive(Component)]
pub struct Chunk {
    cell_size: f32,
}

#[derive(Resource, InspectorOptions, Reflect, PartialEq, Clone)]
#[reflect(InspectorOptions)]
pub struct ChunksConfig {
    size: f32,
    cell_size: f32,
    render_distance: u32,
    updates_per_frame: usize,
    lod_breakpoints: HashMap<u32, f32>,
    load_chunks: bool,
}

impl ChunksConfig {
    pub fn get_cell_size(&self, chunk: GridCoordinates, camera: GridCoordinates) -> f32 {
        let distance = chunk.distance(camera).round() as u32;

        let breakpoint = self
            .lod_breakpoints
            .keys()
            .filter(|k| **k <= distance)
            .max()
            .unwrap_or_else(|| self.lod_breakpoints.keys().max().unwrap());

        *self.lod_breakpoints.get(breakpoint).unwrap()
    }
}

impl Default for ChunksConfig {
    fn default() -> Self {
        Self {
            size: 64.0,
            cell_size: 0.5,
            render_distance: 40,
            updates_per_frame: 4,
            lod_breakpoints: [(0, 0.5), (4, 1.0), (8, 2.0), (16, 4.0)]
                .into_iter()
                .collect(),
            load_chunks: true,
        }
    }
}

#[derive(Debug, Component)]
struct LoadChunk;

#[derive(Component)]
struct ComputeMesh(Task<Mesh>);

#[derive(Component)]
struct DespawnChunk;
