use crate::vertex::Vertex;
use glium::index::PrimitiveType;

/// A Grid is a drawable thing too.
#[derive(Copy, Clone)]
pub struct Grid {
    pub scale: u16,
    pub square_count: u16,
    pub vertices_count: u16,
    pub vertices: [Vertex; 516],
    pub max_scale: u16,
    pub translation: [f32; 3],
    pub rotation: [f32; 3],
    pub color: [f32; 4],
    pub fluid: i32,
    pub noise: i32,
    pub key: u64,
}

use crate::drawable::Drawable;

impl Grid {
    /// Create a new default grid
    pub const fn new() -> Grid {
        Grid {
            scale: 128,
            square_count: 16384,  // self.scale * self.scale
            vertices_count: 1548, // 2 * (6 * (self.scale+1))
            vertices: [Vertex {
                position: [0.0, 0.0, 0.0],
                normal: [0.0, 0.0, 0.0],
            }; 516],
            max_scale: 200,
            translation: [0.0; 3],
            rotation: [0.0; 3],
            color: [0.5, 0.5, 0.5, 0.1],
            fluid: 0,
            noise: 0,
            key: 0,
        }
    }
}

impl Drawable for Grid {
    /// Init a grid once it is created.
    fn init(&mut self) {
        let mut index = 0;
        let mut increment = || -> usize {
            let result = index;
            index += 1;
            result
        };

        let row_vertices: [f32; 6] = [
            -1.0, 1.0, 0.0, // top left
            1.0, 1.0, 0.0, // top right
        ];
        let col_vertices: [f32; 6] = [
            -1.0, 1.0, 0.0, // top left
            -1.0, -1.0, 0.0, // bottom left
        ];

        if self.scale > self.max_scale {
            panic!("Scale for grid is out of bounds");
        }
        // We want one pair of vertices for each row +1 and one for each column + 1

        let scale_f = self.scale as f32;
        for row in 0..=self.scale {
            self.vertices[increment()] = Vertex {
                position: [
                    row_vertices[0] * scale_f / 2.0,
                    (-scale_f) / 2.0 + row as f32,
                    (row_vertices[2]) * scale_f / 2.0,
                ],
                normal: [0.0, 1.0, 0.0],
            };
            self.vertices[increment()] = Vertex {
                position: [
                    (row_vertices[3]) * scale_f / 2.0,
                    (-scale_f) / 2.0 + row as f32,
                    (row_vertices[5]) * scale_f / 2.0,
                ],
                normal: [0.0, 1.0, 0.0],
            };
        }

        for col in 0..=self.scale {
            self.vertices[increment()] = Vertex {
                position: [
                    (-scale_f) / 2.0 + col as f32,
                    (col_vertices[1]) * scale_f / 2.0,
                    (col_vertices[2]) * scale_f / 2.0,
                ],
                normal: [0.0, 1.0, 0.0],
            };
            self.vertices[increment()] = Vertex {
                position: [
                    (-scale_f) / 2.0 + col as f32,
                    (col_vertices[4]) * scale_f / 2.0,
                    (col_vertices[5]) * scale_f / 2.0,
                ],
                normal: [0.0, 1.0, 0.0],
            };
        }

        self.square_count = self.scale * self.scale;
        self.vertices_count = 2 * (6 * (self.scale + 1));

        self.key = rand::random();
    }
    fn key(&self) -> u64 {
        self.key
    }

    /// We calculated the number of vertices after we created it.
    fn count_vertices(&self) -> u16 {
        self.vertices_count
    }

    /// Where is the grid.
    fn translation(&self) -> &[f32; 3] {
        &self.translation
    }

    fn primitive_type(&self) -> glium::index::PrimitiveType {
        PrimitiveType::LinesList
    }

    /// Move the grid.
    fn translate(&mut self, amount: [f32; 3]) {
        self.translation[0] += amount[0];
        self.translation[1] += amount[1];
        self.translation[2] += amount[2];
    }

    /// Rotate the grid.
    fn rotate(&mut self, amount: [f32; 3]) {
        self.rotation[0] += amount[0];
        self.rotation[1] += amount[1];
        self.rotation[2] += amount[2];
    }

    /// How is the grid rotated?
    fn rotation(&self) -> &[f32; 3] {
        &self.rotation
    }

    /// Tell me the vertices to draw.
    fn vertices(&self) -> Vec<Vertex> {
        self.vertices.to_vec()
    }

    /// What color are the lines?
    fn color(&self) -> &[f32; 4] {
        &self.color
    }

    fn fluid(&self) -> i32 {
        self.fluid
    }

    fn noise(&self) -> i32 {
        self.noise
    }

    /// Calculate the distance from the camera to the grid.
    fn depth(&self, camera: [f32; 3]) -> f32 {
        ((self.translation[0] - camera[0]).powi(2)
            + (self.translation[1] - camera[1]).powi(2)
            + (self.translation[2] - camera[2]).powi(2))
        .sqrt()
    }
}
