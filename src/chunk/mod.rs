use bevy::{
    prelude::*,
    render::{mesh::Indices, primitives::Aabb, render_resource::PrimitiveTopology},
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use noise::NoiseFn;

use crate::{
    camera_controller::CameraController, learn_shaders::MaterialConfig,
    noise_graph::NoiseGraphResource, pause::GameState,
};

use self::grid::{ChunkGrid, ChunkGridPlugin, GridCoordinates};

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
                    .with_system(unload_chunks), //.with_system(add_center_point_to_chunks)
            )
            .add_system_set(SystemSet::on_enter(GameState::Running).with_system(reload_chunks));
    }
}

fn trigger_chunk_creation(
    query: Query<&GridCoordinates, With<CameraController>>,
    chunk_grid: Res<ChunkGrid>,
    config: Res<ChunksConfig>,
    mut commands: Commands,
    material_config: Res<MaterialConfig>,
) {
    if config.load_chunks {
        let render_distance = config.render_distance;

        if let Ok(camera_coordinates) = query.get_single() {
            let mut load_chunk = move |x, z| {
                let chunk_coordinates = *camera_coordinates + GridCoordinates::new(x, z);

                if chunk_coordinates.distance_squared(*camera_coordinates) <= render_distance.pow(2)
                    && !chunk_grid.contains(&chunk_coordinates)
                {
                    chunk_grid.insert(chunk_coordinates);
                    commands.spawn((
                        chunk_coordinates,
                        Chunk {
                            cell_size: config.get_cell_size(chunk_coordinates, *camera_coordinates),
                        },
                        LoadChunk,
                        Transform::from_translation(
                            chunk_coordinates.to_translation(config.size as i32),
                        ),
                        GlobalTransform::default(),
                        VisibilityBundle {
                            visibility: Visibility::VISIBLE,
                            computed: ComputedVisibility::default(),
                        },
                        material_config.chunk_material.clone(),
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
    noise_graph: Res<NoiseGraphResource>,
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
            generate_chunk_data(size, cell_size, translation, |x, z| {
                noise.get([x as f64, z as f64]) as f32
            })
        });
        let mut entity = commands.entity(entity);
        entity.insert(ComputeMesh(task)).remove::<LoadChunk>();
    }
}

fn insert_mesh(
    mut query: Query<(Entity, &mut ComputeMesh)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<ChunksConfig>,
) {
    for (entity, mut task) in query.iter_mut().take(config.updates_per_frame) {
        if let Some(ChunkData { mesh, aabb }) = future::block_on(future::poll_once(&mut task.0)) {
            let mut entity = commands.entity(entity);
            entity.remove::<ComputeMesh>();
            entity.remove::<Handle<Mesh>>();
            entity.insert((meshes.add(mesh), aabb));
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
        for (entity, mut chunk, coordinates) in chunks.iter_mut()
        /* .take(config.updates_per_frame) */
        {
            let new_cell_size = config.get_cell_size(*coordinates, *camera_coordinates);

            if chunk.cell_size != new_cell_size {
                chunk.cell_size = new_cell_size;
                commands.entity(entity).insert(LoadChunk);
            }
        }
    }
}

fn reload_chunks(
    mut query: Query<Entity, (Without<LoadChunk>, With<Chunk>)>,
    mut commands: Commands,
) {
    query.for_each_mut(|entity| {
        commands.entity(entity).insert(LoadChunk);
    })
}

fn unload_chunks(
    mut commands: Commands,
    chunks: Query<(Entity, &GridCoordinates), (With<Chunk>, Without<ComputeMesh>)>,
    camera: Query<&GridCoordinates, With<CameraController>>,
    chunks_config: Res<ChunksConfig>,
    chunk_grid: Res<ChunkGrid>,
) {
    if chunks_config.load_chunks {
        if let Ok(camera) = camera.get_single() {
            chunks.for_each(|(entity, coordinates)| {
                let is_outside_render_distance =
                    camera.distance_squared(*coordinates) > chunks_config.render_distance.pow(2);

                if is_outside_render_distance {
                    commands.entity(entity).despawn_recursive();
                    chunk_grid.remove(coordinates);
                }
            });
        }
    }
}

fn generate_chunk_data(
    size: f32,
    cell_size: f32,
    position: Vec3,
    compute_height: impl FnMut(f32, f32) -> f32,
) -> ChunkData {
    assert!(size % cell_size == 0.0);
    let cells_per_side = (size / cell_size) as usize;
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let vertices = vertices(cell_size, cells_per_side, position, compute_height);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.set_indices(Some(Indices::U32(indices(cells_per_side))));
    mesh.duplicate_vertices();
    mesh.set_indices(None);
    mesh.compute_flat_normals();

    let aabb = mesh.compute_aabb().expect("Failed to compute Mesh Aabb");

    ChunkData { mesh, aabb }
}

fn vertices(
    cell_size: f32,
    cells_per_side: usize,
    position: Vec3,
    mut compute_height: impl FnMut(f32, f32) -> f32,
) -> Vec<[f32; 3]> {
    let mut vertices = Vec::with_capacity((cells_per_side + 1) * (cells_per_side + 1));
    let cells_per_direction = cells_per_side as isize / 2;

    for x_index in -cells_per_direction..=cells_per_direction {
        for z_index in -cells_per_direction..=cells_per_direction {
            let x = x_index as f32 * cell_size;
            let z = z_index as f32 * cell_size;
            let y = compute_height(x + position.x, z + position.z);

            vertices.push([x, y, z]);
        }
    }

    vertices
}

fn indices(cells_per_side: usize) -> Vec<u32> {
    let mut indices = Vec::with_capacity(cells_per_side * cells_per_side * 6);
    let cells_per_side = cells_per_side as u32;

    for x in 0..cells_per_side {
        for z in 0..cells_per_side {
            indices.extend([
                x * (cells_per_side + 1) + z,
                x * (cells_per_side + 1) + z + 1,
                (x + 1) * (cells_per_side + 1) + z + 1,
                (x + 1) * (cells_per_side + 1) + z + 1,
                (x + 1) * (cells_per_side + 1) + z,
                x * (cells_per_side + 1) + z,
            ]);
        }
    }

    indices
}

#[derive(Component)]
pub struct Chunk {
    cell_size: f32,
}

#[derive(Resource, Reflect, PartialEq, Clone)]
pub struct ChunksConfig {
    size: f32,
    cell_size: f32,
    render_distance: u32,
    updates_per_frame: usize,
    lod_breakpoints: Vec<(u32, f32)>,
    load_chunks: bool,
}

impl ChunksConfig {
    pub fn get_cell_size(&self, chunk: GridCoordinates, camera: GridCoordinates) -> f32 {
        let distance = chunk.distance(camera).round() as u32;
        let lowest_breakpoint = self
            .lod_breakpoints
            .first()
            .expect("Expected at least 1 lod_breakpoint");
        let highest_breakpoint = self
            .lod_breakpoints
            .last()
            .expect("Expected at least 1 lod_breakpoint");

        if distance <= lowest_breakpoint.0 {
            return lowest_breakpoint.1;
        } else if distance >= highest_breakpoint.0 {
            return highest_breakpoint.1;
        } else {
            for window in self.lod_breakpoints.windows(2) {
                if window[0].0 <= distance && window[1].0 > distance {
                    return window[0].1;
                }
            }

            unreachable!();
        }
    }
}

impl Default for ChunksConfig {
    fn default() -> Self {
        Self {
            size: 128.0,
            cell_size: 0.5,
            render_distance: 40,
            updates_per_frame: 4,
            lod_breakpoints: vec![
                (0, 0.5),
                (4, 1.0),
                (8, 4.0),
                (16, 8.0),
                (24, 16.0),
                (32, 16.0),
                (48, 32.0),
            ],
            load_chunks: true,
        }
    }
}

#[derive(Debug, Component)]
struct LoadChunk;

#[derive(Component)]
struct ComputeMesh(Task<ChunkData>);

struct ChunkData {
    mesh: Mesh,
    aabb: Aabb,
}

#[derive(Component)]
struct DespawnChunk;
