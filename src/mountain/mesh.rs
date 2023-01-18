use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}};

pub fn create_mesh(
    size: f32,
    cell_size: f32,
    position: Vec3,
    compute_height: impl FnMut(f32, f32) -> f32,
) -> Mesh {
    assert!(size % cell_size == 0.0);
    let cells_per_side = (size / cell_size) as usize;
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices(cell_size, cells_per_side, position, compute_height));
    mesh.set_indices(Some(Indices::U32(indices(cells_per_side))));
    mesh.duplicate_vertices();
    mesh.set_indices(None);
    mesh.compute_flat_normals();
    mesh
}

fn vertices(cell_size: f32, cells_per_side: usize, position: Vec3, mut compute_height: impl FnMut(f32, f32) -> f32) -> Vec<[f32; 3]> {
    let mut vertices = Vec::with_capacity((cells_per_side+1) * (cells_per_side+1));
    let cells_per_direction = cells_per_side as isize / 2;

    for x_index in -cells_per_direction..=cells_per_direction {
        for z_index in -cells_per_direction..=cells_per_direction {
            let x = x_index as f32 * cell_size + position.x;
            let z = z_index as f32 * cell_size + position.z;
            let y = compute_height(x, z) + position.y;

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
                x * (cells_per_side+1) + z,
                x * (cells_per_side+1) + z + 1,
                (x+1) * (cells_per_side+1) + z + 1,
                (x+1) * (cells_per_side+1) + z + 1,
                (x+1) * (cells_per_side+1) + z,
                x * (cells_per_side+1) + z,
            ]);
        }
    }

    indices
}
