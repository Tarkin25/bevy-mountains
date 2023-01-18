use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_inspector_egui::{
    prelude::*,
    quick::{ResourceInspectorPlugin, WorldInspectorPlugin},
};
use futures_lite::future;

use crate::{camera_controller::CameraController, pause::GameState};

use self::manager::{ChunkManager, ChunkState, GridCoordinates};

use super::{
    mesh::create_mesh,
    terrain_noise::{GeneralNoiseConfig, TerrainGenerator},
};

mod manager;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateChunk>()
            .init_resource::<ChunkManager>()
            .init_resource::<ChunksConfig>()
            .register_type::<ChunksConfig>()
            .add_plugin(ResourceInspectorPlugin::<ChunksConfig>::default())
            .add_plugin(WorldInspectorPlugin)
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(trigger_chunk_creation)
                    .with_system(spawn_compute_mesh_tasks)
                    .with_system(insert_mesh)
                    .with_system(unload_chunks)
                    .with_system(despawn_chunks),
            );
    }
}

fn trigger_chunk_creation(
    query: Query<&Transform, With<CameraController>>,
    mut events: EventWriter<CreateChunk>,
    chunk_manager: Res<ChunkManager>,
    config: Res<ChunksConfig>,
) {
    let render_distance = config.render_distance as i32;
    let chunk_size = config.size as i32;
    let player_grid_coordinates =
        GridCoordinates::from_translation(query.single().translation, chunk_size);

    for x in -render_distance..=render_distance {
        for z in -render_distance..=render_distance {
            let chunk_grid_coordinates =
                player_grid_coordinates + IVec2::new(x * chunk_size, z * chunk_size);

            if !chunk_manager.contains_key(&chunk_grid_coordinates) {
                events.send(CreateChunk {
                    size: config.size,
                    cell_size: config.cell_size,
                    grid_coordinates: chunk_grid_coordinates,
                });
            }
        }
    }
}

fn spawn_compute_mesh_tasks(
    mut events: EventReader<CreateChunk>,
    terrain_generator: Res<TerrainGenerator>,
    mut commands: Commands,
    noise_config: Res<GeneralNoiseConfig>,
    chunk_manager: Res<ChunkManager>,
) {
    let pool = AsyncComputeTaskPool::get();

    for event in events.iter().take(20) {
        let CreateChunk {
            size,
            cell_size,
            grid_coordinates,
        } = *event;
        let terrain_generator = terrain_generator.clone();
        let translation = grid_coordinates.into();
        let amplitude = noise_config.amplitude;
        let scale = noise_config.scale;
        let task = pool.spawn(async move {
            create_mesh(size, cell_size, translation, |x, z| {
                terrain_generator.compute_height(amplitude, scale, [x, z])
            })
        });
        commands
            .spawn(Transform::from_translation(translation))
            .insert(ComputeMesh(task))
            .insert(Chunk { size, cell_size })
            .insert(grid_coordinates);
        chunk_manager.insert(grid_coordinates, ChunkState::Creating);
    }
}

fn insert_mesh(
    mut query: Query<(Entity, &mut ComputeMesh, &GridCoordinates)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunk_manager: Res<ChunkManager>,
) {
    query.for_each_mut(|(entity, mut task, grid_coordinates)| {
        if let Some(mesh) = future::block_on(future::poll_once(&mut task.0)) {
            let mut entity = commands.entity(entity);
            entity.remove::<ComputeMesh>();
            entity.insert(MaterialMeshBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::PURPLE.into()),
                ..Default::default()
            });
            chunk_manager.insert(*grid_coordinates, ChunkState::Loaded);
        }
    });
}

fn unload_chunks(
    mut commands: Commands,
    chunks: Query<(Entity, &GridCoordinates)>,
    camera: Query<&Transform, With<CameraController>>,
    chunk_manager: Res<ChunkManager>,
    chunks_config: Res<ChunksConfig>,
) {
    let camera_grid_coordinates = GridCoordinates::from_translation(camera.single().translation, chunks_config.size as i32);
    let bounds_distance = (chunks_config.size as u32 * chunks_config.render_distance) as i32;

    chunks.for_each(|(entity, coordinates)| {
        let is_outside_pos_x = camera_grid_coordinates.x + bounds_distance < coordinates.x;
        let is_outside_neg_x = camera_grid_coordinates.x - bounds_distance > coordinates.x;
        let is_outside_pos_z = camera_grid_coordinates.y + bounds_distance < coordinates.y;
        let is_outside_neg_z = camera_grid_coordinates.y - bounds_distance > coordinates.y;
        
        let is_outside_render_distance = is_outside_pos_x || is_outside_neg_x || is_outside_pos_z || is_outside_neg_z;

        if is_outside_render_distance && chunk_manager.contains_key(coordinates) {
            chunk_manager.remove(coordinates);
            commands.entity(entity).insert(DespawnChunk);
        }
    });
}

fn despawn_chunks(
    mut commands: Commands,
    chunks: Query<Entity, (With<DespawnChunk>, Without<ComputeMesh>)>
) {
    chunks.iter().take(20).for_each(|entity| {
        commands.entity(entity).despawn();
    });
}

#[derive(Component)]
pub struct Chunk {
    size: f32,
    cell_size: f32,
}

#[derive(Resource, InspectorOptions, Reflect, PartialEq, Clone)]
#[reflect(InspectorOptions)]
pub struct ChunksConfig {
    size: f32,
    cell_size: f32,
    render_distance: u32,
}

impl Default for ChunksConfig {
    fn default() -> Self {
        Self {
            size: 100.0,
            cell_size: 0.5,
            render_distance: 1,
        }
    }
}

#[derive(Debug)]
struct CreateChunk {
    size: f32,
    cell_size: f32,
    grid_coordinates: GridCoordinates,
}

#[derive(Component)]
struct ComputeMesh(Task<Mesh>);

#[derive(Component)]
struct DespawnChunk;
