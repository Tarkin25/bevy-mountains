use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    utils::HashMap,
};
use bevy_inspector_egui::{
    prelude::*,
    quick::{ResourceInspectorPlugin, WorldInspectorPlugin},
};
use futures_lite::future;

use crate::{camera_controller::CameraController, pause::GameState};

use self::grid::{ChunkGrid, ChunkGridPlugin, GridCoordinates};

use super::{
    mesh::create_mesh,
    terrain_noise::{GeneralNoiseConfig, TerrainGenerator},
};

mod grid;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunksConfig>()
            .register_type::<ChunksConfig>()
            .add_plugin(ResourceInspectorPlugin::<ChunksConfig>::default())
            .add_plugin(WorldInspectorPlugin)
            .add_plugin(ChunkGridPlugin)
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(trigger_chunk_creation.before(spawn_compute_mesh_tasks))
                    .with_system(update_level_of_detail.before(spawn_compute_mesh_tasks))
                    .with_system(spawn_compute_mesh_tasks.before(insert_mesh))
                    .with_system(insert_mesh.before(unload_chunks))
                    .with_system(unload_chunks)
                    .with_system(add_center_point_to_chunks)
            )
            .add_system_set(SystemSet::on_enter(GameState::Running).with_system(reload_chunks));
    }
}

fn add_center_point_to_chunks(query: Query<Entity, Added<Chunk>>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, mut commands: Commands) {
    query.for_each(|entity| {
        let child = commands.spawn(PbrBundle {
            material: materials.add(Color::LIME_GREEN.into()),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            ..Default::default()
        }).id();
        
        commands.entity(entity).add_child(child);
    })
}

fn trigger_chunk_creation(
    query: Query<&GridCoordinates, With<CameraController>>,
    chunk_grid: Res<ChunkGrid>,
    config: Res<ChunksConfig>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if config.load_chunks {
        let render_distance = config.render_distance as i32;

        if let Ok(camera_coordinates) = query.get_single() {
            let mut load_chunk = move |x, z| {
                let chunk_coordinates = *camera_coordinates + IVec2::new(x, z);
    
                if !chunk_grid.contains(&chunk_coordinates) {
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
                        materials.add(Color::PURPLE.into()),
                    ));
                }
            };
    
            for x in (0..=render_distance).rev() {
                for z in (0..=render_distance).rev() {
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
    terrain_generator: Res<TerrainGenerator>,
    mut commands: Commands,
    noise_config: Res<GeneralNoiseConfig>,
    query: Query<(Entity, &GridCoordinates, &Chunk), With<LoadChunk>>,
    chunks_config: Res<ChunksConfig>,
) {
    let pool = AsyncComputeTaskPool::get();

    for (entity, grid_coordinates, chunk) in query.iter().take(chunks_config.updates_per_frame) {
        let grid_coordinates = *grid_coordinates;
        let Chunk { cell_size } = *chunk;
        let terrain_generator = terrain_generator.clone();
        let translation = grid_coordinates.to_translation(chunks_config.size as i32);
        let size = chunks_config.size;
        let amplitude = noise_config.amplitude;
        let scale = noise_config.scale;
        let task = pool.spawn(async move {
            create_mesh(size, cell_size, translation, |x, z| {
                //terrain_generator.compute_height(amplitude, scale, [x, z])
                0.0
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
            entity.insert(meshes.add(mesh));
        }
    }
}

fn update_level_of_detail(
    mut chunks: Query<(Entity, &mut Chunk, &GridCoordinates), Without<LoadChunk>>,
    mut commands: Commands,
    camera: Query<&GridCoordinates, With<CameraController>>,
    config: Res<ChunksConfig>,
) {
    if let Ok(camera_coordinates) = camera.get_single() {
        for (entity, mut chunk, coordinates) in chunks.iter_mut().take(config.updates_per_frame) {
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
    camera: Query<&Transform, With<CameraController>>,
    chunks_config: Res<ChunksConfig>,
    chunk_grid: Res<ChunkGrid>,
) {
    if chunks_config.load_chunks {
        let camera_grid_coordinates = GridCoordinates::from_translation(
            camera.single().translation,
            chunks_config.size as i32,
        );
        let bounds_distance = (chunks_config.size as u32 * chunks_config.render_distance) as i32;

        chunks.for_each(|(entity, coordinates)| {
            let is_outside_pos_x = camera_grid_coordinates.x + bounds_distance < coordinates.x;
            let is_outside_neg_x = camera_grid_coordinates.x - bounds_distance > coordinates.x;
            let is_outside_pos_z = camera_grid_coordinates.y + bounds_distance < coordinates.y;
            let is_outside_neg_z = camera_grid_coordinates.y - bounds_distance > coordinates.y;

            let is_outside_render_distance =
                is_outside_pos_x || is_outside_neg_x || is_outside_pos_z || is_outside_neg_z;

            if is_outside_render_distance {
                commands.entity(entity).despawn_recursive();
                chunk_grid.remove(coordinates);
            }
        });
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
        let dx = chunk.0.x.abs_diff(camera.0.x);
        let dz = chunk.0.y.abs_diff(camera.0.y);
        let d_max = dx.max(dz);

        let breakpoint = self
            .lod_breakpoints
            .keys()
            .filter(|k| **k <= d_max)
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
            render_distance: 0,
            updates_per_frame: 1,
            lod_breakpoints: [(0, 0.5), (1, 1.0), (2, 2.0), (3, 4.0)]
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
