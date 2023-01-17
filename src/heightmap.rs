use std::ops::{Index, IndexMut};

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

#[derive(Debug, Component)]
pub struct Heightmap {
    size: f32,
    square_size: f32,
    squares_per_side: usize,
    heights: Vec<Vec<f32>>,
}

// Creation, read & write access
impl Heightmap {
    pub fn new(size: f32, square_size: f32) -> Self {
        assert!(size % square_size == 0.0);
        let squares_per_side = (size / square_size) as usize;
        let heights = vec![vec![0.0; squares_per_side + 1]; squares_per_side + 1];

        Self {
            size,
            square_size,
            squares_per_side,
            heights,
        }
    }

    pub fn get(&self, x: f32, z: f32) -> Option<&f32> {
        let x_index = self.get_index(x)?;
        let z_index = self.get_index(z)?;

        self.heights.get(x_index).and_then(|z| z.get(z_index))
    }

    pub fn get_mut(&mut self, x: f32, z: f32) -> Option<&mut f32> {
        let x_index = self.get_index(x)?;
        let z_index = self.get_index(z)?;

        self.heights
            .get_mut(x_index)
            .and_then(|z| z.get_mut(z_index))
    }

    fn get_index(&self, n: f32) -> Option<usize> {
        if n % self.square_size == 0.0 {
            Some((n / self.square_size) as usize)
        } else {
            None
        }
    }

    fn get_index_unchecked(&self, n: f32) -> usize {
        assert!(n % self.square_size == 0.0);
        (n / self.square_size) as usize
    }

    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn square_size(&self) -> f32 {
        self.square_size
    }

    pub fn squares_per_side(&self) -> usize {
        self.squares_per_side
    }
}

impl Index<[f32; 2]> for Heightmap {
    type Output = f32;

    fn index(&self, [x, z]: [f32; 2]) -> &Self::Output {
        let x_index = self.get_index_unchecked(x);
        let z_index = self.get_index_unchecked(z);

        &self.heights[x_index][z_index]
    }
}

impl IndexMut<[f32; 2]> for Heightmap {
    fn index_mut(&mut self, [x, z]: [f32; 2]) -> &mut Self::Output {
        let x_index = self.get_index_unchecked(x);
        let z_index = self.get_index_unchecked(z);

        &mut self.heights[x_index][z_index]
    }
}

// Mesh generation
impl Heightmap {
    pub fn compute_mesh(&self) -> Mesh {
        let vertices = self.vertices();
        let indices = self.indices();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.duplicate_vertices();
        mesh.set_indices(None);
        mesh.compute_flat_normals();

        mesh
    }

    fn vertices(&self) -> Vec<[f32; 3]> {
        let mut vertices =
            Vec::with_capacity((self.squares_per_side() + 1) * (self.squares_per_side() + 1));

        for x in 0..=self.squares_per_side() {
            for z in 0..=self.squares_per_side() {
                vertices.push([
                    x as f32 * self.square_size(),
                    self.heights[x][z],
                    z as f32 * self.square_size(),
                ]);
            }
        }

        vertices
    }

    fn indices(&self) -> Vec<u32> {
        let mut indices = Vec::with_capacity(self.squares_per_side() * self.squares_per_side() * 6);
        let squares_per_side = self.squares_per_side() as u32;

        for x in 0..squares_per_side {
            for z in 0..squares_per_side {
                indices.extend([
                    x * (squares_per_side + 1) + z,
                    x * (squares_per_side + 1) + z + 1,
                    (x + 1) * (squares_per_side + 1) + z + 1,
                    (x + 1) * (squares_per_side + 1) + z + 1,
                    (x + 1) * (squares_per_side + 1) + z,
                    x * (squares_per_side + 1) + z,
                ]);
            }
        }

        indices
    }
}

// Height computation
impl Heightmap {
    pub fn compute_heights(&mut self, mut f: impl FnMut(f32, f32) -> f32) {
        for x_index in 0..=self.squares_per_side() {
            for z_index in 0..=self.squares_per_side() {
                let x = x_index as f32 * self.square_size();
                let z = z_index as f32 * self.square_size();
                self.heights[x_index][z_index] = f(x, z);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertices_works() {
        let map = Heightmap::new(1.0, 1.0);
        assert_eq!(
            map.vertices(),
            vec![
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 1.0],
            ]
        );
    }

    #[test]
    fn indices_works() {
        let map = Heightmap::new(1.0, 1.0);
        assert_eq!(map.indices(), vec![0, 1, 3, 3, 2, 0,]);

        let map = Heightmap::new(2.0, 1.0);
        assert_eq!(
            map.indices(),
            vec![0, 1, 4, 4, 3, 0, 1, 2, 5, 5, 4, 1, 3, 4, 7, 7, 6, 3, 4, 5, 8, 8, 7, 4,]
        );
    }
}
