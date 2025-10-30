use crate::vertex::Vertex;
use glium::index::PrimitiveType;
use nalgebra::Isometry3;
use nalgebra::Point3;
use nalgebra::Vector3;

/// A cube is a drawable item that can be positioned, rotated and scaled.
#[derive(Copy, Clone)]
pub struct Cube {
    pub vertices_count: u16,
    pub translation: [f32; 3],
    pub rotation: [f32; 3],
    pub color: [f32; 4],
    pub scale: f32,
    pub center: f32,
    pub floor: f32,
    pub fluid: i32,
    pub noise: i32,
    pub bottom_occluded: bool,
    pub left_occluded: bool,
    pub right_occluded: bool,
    pub front_occluded: bool,
    pub back_occluded: bool,
    pub top_occluded: bool,
    pub smooth: bool,
    pub key: u64,
}

use nalgebra_glm::Vec3;

use crate::drawable::Drawable;

impl Cube {
    /// Create a new default cube.
    pub const fn new() -> Cube {
        Cube {
            vertices_count: 216,
            translation: [0.0; 3],
            rotation: [0.0; 3],
            color: [0.3, 0.3, 0.1, 1.0],
            scale: 0.9999, // The scale is slightly smaller than 1 to prevent z-fighting
            center: 0.5,
            floor: 0.0001,
            fluid: 0,
            noise: 0,
            bottom_occluded: false,
            left_occluded: false,
            right_occluded: false,
            front_occluded: false,
            back_occluded: false,
            top_occluded: false,
            smooth: false,
            key: 0,
        }
    }
}

impl Drawable for Cube {
    /// Init a new cube so it's ready to draw.
    fn init(&mut self) {
        self.key = rand::random();
    }

    fn material_key(&self) -> String {
        format!(
            "cube_{}_{}_{}_{}_{}_{}",
            self.fluid, self.noise, self.color[0], self.color[1], self.color[2], self.color[3]
        )
    }

    fn key(&self) -> u64 {
        self.key
    }

    /// A cube always has the same number of vertices minus oclusion
    fn count_vertices(&self) -> u16 {
        let mut occluded = self.vertices_count;
        if self.bottom_occluded {
            occluded -= 36;
        }
        if self.left_occluded {
            occluded -= 36;
        }
        if self.right_occluded {
            occluded -= 36;
        }
        if self.front_occluded {
            occluded -= 36;
        }
        if self.back_occluded {
            occluded -= 36;
        }
        if self.top_occluded {
            occluded -= 36;
        }
        occluded
    }

    fn primitive_type(&self) -> glium::index::PrimitiveType {
        PrimitiveType::TrianglesList
    }

    /// We can move a cube
    fn translation(&self) -> &[f32; 3] {
        &self.translation
    }

    /// Cubes have a colour - including alphas.
    fn color(&self) -> &[f32; 4] {
        &self.color
    }

    fn fluid(&self) -> i32 {
        self.fluid
    }

    fn noise(&self) -> i32 {
        self.noise
    }

    /// Move a cube.
    fn translate(&mut self, amount: [f32; 3]) {
        self.translation[0] += amount[0];
        self.translation[1] += amount[1];
        self.translation[2] += amount[2];
    }

    /// Rotate a cube.
    fn rotate(&mut self, amount: [f32; 3]) {
        self.rotation[0] += amount[0];
        self.rotation[1] += amount[1];
        self.rotation[2] += amount[2];
    }

    /// Get the current rotation.
    fn rotation(&self) -> &[f32; 3] {
        &self.rotation
    }

    /// Get an array of vertices.
    fn vertices(&self) -> Vec<Vertex> {
        // We really have 8 points.
        // Start by calcuting the points.
        // naming is l/r u/d f/b
        // which is -x/+x -y/+y / -z/+z
        let bulge = 0.6;
        let lc = [
            if self.smooth
                && !self.front_occluded
                && !self.bottom_occluded
                && !self.left_occluded
                && !self.back_occluded
                && !self.top_occluded
            {
                self.center - self.center * bulge
            } else {
                self.floor
            },
            self.center,
            self.center,
        ];
        let rc = [
            if self.smooth
                && !self.front_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.back_occluded
                && !self.top_occluded
            {
                self.center + self.center * bulge
            } else {
                self.scale
            },
            self.center,
            self.center,
        ];
        let fc = [
            self.center,
            self.center,
            if self.smooth
                && !self.front_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.top_occluded
            {
                self.center - self.center * bulge
            } else {
                self.floor
            },
        ];
        let bc = [
            self.center,
            self.center,
            if self.smooth
                && !self.back_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.top_occluded
            {
                self.center + self.center * bulge
            } else {
                self.scale
            },
        ];
        let dc = [
            self.center,
            if self.smooth
                && !self.back_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.front_occluded
            {
                self.center - self.center * bulge
            } else {
                self.floor
            },
            self.center,
        ];

        let uc = [
            self.center,
            if self.smooth
                && !self.back_occluded
                && !self.top_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.front_occluded
            {
                self.center + self.center * bulge
            } else {
                self.scale
            },
            self.center,
        ];

        let ldf = [
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * bulge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * bulge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * bulge
            } else {
                self.floor
            },
        ];
        let luf = [
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.left_occluded {
                self.center - self.center * bulge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.left_occluded {
                self.center + self.center * bulge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.left_occluded {
                self.center - self.center * bulge
            } else {
                self.floor
            },
        ];
        let ldb = [
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * bulge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * bulge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center + self.center * bulge
            } else {
                self.scale
            },
        ];
        let lub = [
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.left_occluded {
                self.center - self.center * bulge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.left_occluded {
                self.center + self.center * bulge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.left_occluded {
                self.center + self.center * bulge
            } else {
                self.scale
            },
        ];
        let rdf = [
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.right_occluded
            {
                self.center + self.center * bulge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.right_occluded
            {
                self.center - self.center * bulge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.right_occluded
            {
                self.center - self.center * bulge
            } else {
                self.floor
            },
        ];
        let ruf = [
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * bulge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * bulge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.right_occluded {
                self.center - self.center * bulge
            } else {
                self.floor
            },
        ];
        let rdb = [
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.right_occluded {
                self.center + self.center * bulge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.right_occluded {
                self.center - self.center * bulge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.right_occluded {
                self.center + self.center * bulge
            } else {
                self.scale
            },
        ];
        let rub = [
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * bulge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * bulge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * bulge
            } else {
                self.scale
            },
        ];

        let mut index: usize = 0;
        let mut increment = || -> usize {
            let result = index;
            index += 1;
            result
        };

        let mut vertices = [Vertex {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
        }; 72];
        // Bottom
        let b11 = Vec3::new(ldf[0] - dc[0], ldf[1] - dc[1], ldf[2] - dc[2]);
        let b12 = Vec3::new(rdf[0] - dc[0], rdf[1] - dc[1], rdf[2] - dc[2]);
        let bc1 = b11.cross(&b12);
        vertices[increment()] = Vertex {
            position: [ldf[0], ldf[1], ldf[2]],
            normal: [bc1[0], bc1[1], bc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [rdf[0], rdf[1], rdf[2]],
            normal: [bc1[0], bc1[1], bc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [dc[0], dc[1], dc[2]],
            normal: [bc1[0], bc1[1], bc1[2]],
        };

        let b21 = Vec3::new(rdf[0] - dc[0], rdf[1] - dc[1], rdf[2] - dc[2]);
        let b22 = Vec3::new(rdb[0] - dc[0], rdb[1] - dc[1], rdb[2] - dc[2]);
        let bc2 = b21.cross(&b22);
        vertices[increment()] = Vertex {
            position: [rdf[0], rdf[1], rdf[2]],
            normal: [bc2[0], bc2[1], bc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [rdb[0], rdb[1], rdb[2]],
            normal: [bc2[0], bc2[1], bc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [dc[0], dc[1], dc[2]],
            normal: [bc2[0], bc2[1], bc2[2]],
        };

        let b31 = Vec3::new(rdb[0] - dc[0], rdb[1] - dc[1], rdb[2] - dc[2]);
        let b32 = Vec3::new(ldb[0] - dc[0], ldb[1] - dc[1], ldb[2] - dc[2]);
        let bc3 = b31.cross(&b32);
        vertices[increment()] = Vertex {
            position: [rdb[0], rdb[1], rdb[2]],
            normal: [bc3[0], bc3[1], bc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [ldb[0], ldb[1], ldb[2]],
            normal: [bc3[0], bc3[1], bc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [dc[0], dc[1], dc[2]],
            normal: [bc3[0], bc3[1], bc3[2]],
        };

        let b41 = Vec3::new(ldb[0] - dc[0], ldb[1] - dc[1], ldb[2] - dc[2]);
        let b42 = Vec3::new(ldf[0] - dc[0], ldf[1] - dc[1], ldf[2] - dc[2]);
        let bc4 = b41.cross(&b42);
        vertices[increment()] = Vertex {
            position: [ldb[0], ldb[1], ldb[2]],
            normal: [bc4[0], bc4[1], bc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [ldf[0], ldf[1], ldf[2]],
            normal: [bc4[0], bc4[1], bc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [dc[0], dc[1], dc[2]],
            normal: [bc4[0], bc4[1], bc4[2]],
        };

        // Left
        let l11 = Vec3::new(ldf[0] - lc[0], ldf[1] - lc[1], ldf[2] - lc[2]);
        let l12 = Vec3::new(ldb[0] - lc[0], ldb[1] - lc[1], ldb[2] - lc[2]);
        let lc1 = l11.cross(&l12);
        vertices[increment()] = Vertex {
            position: [ldf[0], ldf[1], ldf[2]],
            normal: [lc1[0], lc1[1], lc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [ldb[0], ldb[1], ldb[2]],
            normal: [lc1[0], lc1[1], lc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [lc[0], lc[1], lc[2]],
            normal: [lc1[0], lc1[1], lc1[2]],
        };
        let l21 = Vec3::new(luf[0] - lc[0], luf[1] - lc[1], luf[2] - lc[2]);
        let l22 = Vec3::new(ldf[0] - lc[0], ldf[1] - lc[1], ldf[2] - lc[2]);
        let lc2 = l21.cross(&l22);

        vertices[increment()] = Vertex {
            position: [luf[0], luf[1], luf[2]],
            normal: [lc2[0], lc2[1], lc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [ldf[0], ldf[1], ldf[2]],
            normal: [lc2[0], lc2[1], lc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [lc[0], lc[1], lc[2]],
            normal: [lc2[0], lc2[1], lc2[2]],
        };
        let l31 = Vec3::new(lub[0] - lc[0], lub[1] - lc[1], lub[2] - lc[2]);
        let l32 = Vec3::new(luf[0] - lc[0], luf[1] - lc[1], luf[2] - lc[2]);
        let lc3 = l31.cross(&l32);
        vertices[increment()] = Vertex {
            position: [lub[0], lub[1], lub[2]],
            normal: [lc3[0], lc3[1], lc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [luf[0], luf[1], luf[2]],
            normal: [lc3[0], lc3[1], lc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [lc[0], lc[1], lc[2]],
            normal: [lc3[0], lc3[1], lc3[2]],
        };
        let l41 = Vec3::new(ldb[0] - lc[0], ldb[1] - lc[1], ldb[2] - lc[2]);
        let l42 = Vec3::new(lub[0] - lc[0], lub[1] - lc[1], lub[2] - lc[2]);
        let lc4 = l41.cross(&l42);
        vertices[increment()] = Vertex {
            position: [ldb[0], ldb[1], ldb[2]],
            normal: [lc4[0], lc4[1], lc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [lub[0], lub[1], lub[2]],
            normal: [lc4[0], lc4[1], lc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [lc[0], lc[1], lc[2]],
            normal: [lc4[0], lc4[1], lc4[2]],
        };

        // Right
        let r11 = Vec3::new(rdf[0] - rc[0], rdf[1] - rc[1], rdf[2] - rc[2]);
        let r12 = Vec3::new(ruf[0] - rc[0], ruf[1] - rc[1], ruf[2] - rc[2]);
        let rc1 = r11.cross(&r12);
        vertices[increment()] = Vertex {
            position: [rdf[0], rdf[1], rdf[2]],
            normal: [rc1[0], rc1[1], rc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [ruf[0], ruf[1], ruf[2]],
            normal: [rc1[0], rc1[1], rc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [rc[0], rc[1], rc[2]],
            normal: [rc1[0], rc1[1], rc1[2]],
        };
        let r21 = Vec3::new(ruf[0] - rc[0], ruf[1] - rc[1], ruf[2] - rc[2]);
        let r22 = Vec3::new(rub[0] - rc[0], rub[1] - rc[1], rub[2] - rc[2]);
        let rc2 = r21.cross(&r22);
        vertices[increment()] = Vertex {
            position: [ruf[0], ruf[1], ruf[2]],
            normal: [rc2[0], rc2[1], rc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [rub[0], rub[1], rub[2]],
            normal: [rc2[0], rc2[1], rc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [rc[0], rc[1], rc[2]],
            normal: [rc2[0], rc2[1], rc2[2]],
        };
        let r31 = Vec3::new(rub[0] - rc[0], rub[1] - rc[1], rub[2] - rc[2]);
        let r32 = Vec3::new(rdb[0] - rc[0], rdb[1] - rc[1], rdb[2] - rc[2]);
        let rc3 = r31.cross(&r32);
        vertices[increment()] = Vertex {
            position: [rub[0], rub[1], rub[2]],
            normal: [rc3[0], rc3[1], rc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [rdb[0], rdb[1], rdb[2]],
            normal: [rc3[0], rc3[1], rc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [rc[0], rc[1], rc[2]],
            normal: [rc3[0], rc3[1], rc3[2]],
        };
        let r41 = Vec3::new(rdb[0] - rc[0], rdb[1] - rc[1], rdb[2] - rc[2]);
        let r42 = Vec3::new(rdf[0] - rc[0], rdf[1] - rc[1], rdf[2] - rc[2]);
        let rc4 = r41.cross(&r42);
        vertices[increment()] = Vertex {
            position: [rdb[0], rdb[1], rdb[2]],
            normal: [rc4[0], rc4[1], rc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [rdf[0], rdf[1], rdf[2]],
            normal: [rc4[0], rc4[1], rc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [rc[0], rc[1], rc[2]],
            normal: [rc4[0], rc4[1], rc4[2]],
        };

        // Back
        let b11 = Vec3::new(rdb[0] - bc[0], rdb[1] - bc[1], rdb[2] - bc[2]);
        let b12 = Vec3::new(rub[0] - bc[0], rub[1] - bc[1], rub[2] - bc[2]);
        let bc1 = b11.cross(&b12);
        vertices[increment()] = Vertex {
            position: [ldb[0], ldb[1], ldb[2]],
            normal: [bc1[0], bc1[1], bc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [rdb[0], rdb[1], rdb[2]],
            normal: [bc1[0], bc1[1], bc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [bc[0], bc[1], bc[2]],
            normal: [bc1[0], bc1[1], bc1[2]],
        };
        let b21 = Vec3::new(rdb[0] - bc[0], rdb[1] - bc[1], rdb[2] - bc[2]);
        let b22 = Vec3::new(rub[0] - bc[0], rub[1] - bc[1], rub[2] - bc[2]);
        let bc2 = b21.cross(&b22);
        vertices[increment()] = Vertex {
            position: [rdb[0], rdb[1], rdb[2]],
            normal: [bc2[0], bc2[1], bc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [rub[0], rub[1], rub[2]],
            normal: [bc2[0], bc2[1], bc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [bc[0], bc[1], bc[2]],
            normal: [bc2[0], bc2[1], bc2[2]],
        };
        let b31 = Vec3::new(rub[0] - bc[0], rub[1] - bc[1], rub[2] - bc[2]);
        let b32 = Vec3::new(lub[0] - bc[0], lub[1] - bc[1], lub[2] - bc[2]);
        let bc3 = b31.cross(&b32);
        vertices[increment()] = Vertex {
            position: [rub[0], rub[1], rub[2]],
            normal: [bc3[0], bc3[1], bc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [lub[0], lub[1], lub[2]],
            normal: [bc3[0], bc3[1], bc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [bc[0], bc[1], bc[2]],
            normal: [bc3[0], bc3[1], bc3[2]],
        };
        let b41 = Vec3::new(lub[0] - bc[0], lub[1] - bc[1], lub[2] - bc[2]);
        let b42 = Vec3::new(ldb[0] - bc[0], ldb[1] - bc[1], ldb[2] - bc[2]);
        let bc4 = b41.cross(&b42);
        vertices[increment()] = Vertex {
            position: [lub[0], lub[1], lub[2]],
            normal: [bc4[0], bc4[1], bc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [ldb[0], ldb[1], ldb[2]],
            normal: [bc4[0], bc4[1], bc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [bc[0], bc[1], bc[2]],
            normal: [bc4[0], bc4[1], bc4[2]],
        };

        // Front
        let f11 = Vec3::new(ldf[0] - fc[0], ldf[1] - fc[1], ldf[2] - fc[2]);
        let f12 = Vec3::new(luf[0] - fc[0], luf[1] - fc[1], luf[2] - fc[2]);
        let fc1 = f11.cross(&f12);
        vertices[increment()] = Vertex {
            position: [ldf[0], ldf[1], ldf[2]],
            normal: [fc1[0], fc1[1], fc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [luf[0], luf[1], luf[2]],
            normal: [fc1[0], fc1[1], fc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [fc[0], fc[1], fc[2]],
            normal: [fc1[0], fc1[1], fc1[2]],
        };
        let f21 = Vec3::new(luf[0] - fc[0], luf[1] - fc[1], luf[2] - fc[2]);
        let f22 = Vec3::new(ruf[0] - fc[0], ruf[1] - fc[1], ruf[2] - fc[2]);
        let fc2 = f21.cross(&f22);
        vertices[increment()] = Vertex {
            position: [luf[0], luf[1], luf[2]],
            normal: [fc2[0], fc2[1], fc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [ruf[0], ruf[1], ruf[2]],
            normal: [fc2[0], fc2[1], fc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [fc[0], fc[1], fc[2]],
            normal: [fc2[0], fc2[1], fc2[2]],
        };
        let f31 = Vec3::new(ruf[0] - fc[0], ruf[1] - fc[1], ruf[2] - fc[2]);
        let f32 = Vec3::new(rdf[0] - fc[0], rdf[1] - fc[1], rdf[2] - fc[2]);
        let fc3 = f31.cross(&f32);
        vertices[increment()] = Vertex {
            position: [ruf[0], ruf[1], ruf[2]],
            normal: [fc3[0], fc3[1], fc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [rdf[0], rdf[1], rdf[2]],
            normal: [fc3[0], fc3[1], fc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [fc[0], fc[1], fc[2]],
            normal: [fc3[0], fc3[1], fc3[2]],
        };

        let f41 = Vec3::new(rdf[0] - fc[0], rdf[1] - fc[1], rdf[2] - fc[2]);
        let f42 = Vec3::new(ldf[0] - fc[0], ldf[1] - fc[1], ldf[2] - fc[2]);
        let fc4 = f41.cross(&f42);
        vertices[increment()] = Vertex {
            position: [rdf[0], rdf[1], rdf[2]],
            normal: [fc4[0], fc4[1], fc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [ldf[0], ldf[1], ldf[2]],
            normal: [fc4[0], fc4[1], fc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [fc[0], fc[1], fc[2]],
            normal: [fc4[0], fc4[1], fc4[2]],
        };

        // Top
        let t11 = Vec3::new(luf[0] - uc[0], luf[1] - uc[1], luf[2] - uc[2]);
        let t12 = Vec3::new(lub[0] - uc[0], lub[1] - uc[1], lub[2] - uc[2]);
        let tc1 = t11.cross(&t12);
        vertices[increment()] = Vertex {
            position: [luf[0], luf[1], luf[2]],
            normal: [tc1[0], tc1[1], tc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [lub[0], lub[1], lub[2]],
            normal: [tc1[0], tc1[1], tc1[2]],
        };
        vertices[increment()] = Vertex {
            position: [uc[0], uc[1], uc[2]],
            normal: [tc1[0], tc1[1], tc1[2]],
        };

        let t21 = Vec3::new(lub[0] - uc[0], lub[1] - uc[1], lub[2] - uc[2]);
        let t22 = Vec3::new(rub[0] - uc[0], rub[1] - uc[1], rub[2] - uc[2]);
        let tc2 = t21.cross(&t22);
        vertices[increment()] = Vertex {
            position: [lub[0], lub[1], lub[2]],
            normal: [tc2[0], tc2[1], tc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [rub[0], rub[1], rub[2]],
            normal: [tc2[0], tc2[1], tc2[2]],
        };
        vertices[increment()] = Vertex {
            position: [uc[0], uc[1], uc[2]],
            normal: [tc2[0], tc2[1], tc2[2]],
        };

        let t31 = Vec3::new(rub[0] - uc[0], rub[1] - uc[1], rub[2] - uc[2]);
        let t32 = Vec3::new(ruf[0] - uc[0], ruf[1] - uc[1], ruf[2] - uc[2]);
        let tc3 = t31.cross(&t32);
        vertices[increment()] = Vertex {
            position: [rub[0], rub[1], rub[2]],
            normal: [tc3[0], tc3[1], tc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [ruf[0], ruf[1], ruf[2]],
            normal: [tc3[0], tc3[1], tc3[2]],
        };
        vertices[increment()] = Vertex {
            position: [uc[0], uc[1], uc[2]],
            normal: [tc3[0], tc3[1], tc3[2]],
        };
        let t41 = Vec3::new(ruf[0] - uc[0], ruf[1] - uc[1], ruf[2] - uc[2]);
        let t42 = Vec3::new(luf[0] - uc[0], luf[1] - uc[1], luf[2] - uc[2]);
        let tc4 = t41.cross(&t42);
        vertices[increment()] = Vertex {
            position: [ruf[0], ruf[1], ruf[2]],
            normal: [tc4[0], tc4[1], tc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [luf[0], luf[1], luf[2]],
            normal: [tc4[0], tc4[1], tc4[2]],
        };
        vertices[increment()] = Vertex {
            position: [uc[0], uc[1], uc[2]],
            normal: [tc4[0], tc4[1], tc4[2]],
        };

        let bottom = &vertices[0..12];
        let left = &vertices[12..24];
        let right = &vertices[24..36];
        let back = &vertices[36..48];
        let front = &vertices[48..60];
        let top = &vertices[60..72];
        let mut valid = vec![];

        if !self.bottom_occluded {
            valid.extend_from_slice(bottom);
        }
        if !self.left_occluded {
            valid.extend_from_slice(left);
        }
        if !self.right_occluded {
            valid.extend_from_slice(right);
        }
        if !self.front_occluded {
            valid.extend_from_slice(front);
        }
        if !self.back_occluded {
            valid.extend_from_slice(back);
        }
        if !self.top_occluded {
            valid.extend_from_slice(top);
        }

        valid
    }

    fn vertices_world(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let model_tr = Isometry3::new(
            Vector3::from_row_slice(self.translation()),
            Vector3::from_row_slice(self.rotation()),
        );
        let model_r = Isometry3::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::from_row_slice(self.rotation()),
        );
        for vertex in self.vertices() {
            let mut vertex = vertex;
            let funk = model_tr * Point3::from(vertex.position);
            vertex.position = [funk.x, funk.y, funk.z];

            let funk = model_r * Point3::from(vertex.normal);
            vertex.normal = [funk.x, funk.y, funk.z];
            vertices.push(vertex);
        }

        vertices
    }

    /// Calculate the distance between the cube and the camera.
    fn depth(&self, camera: [f32; 3]) -> f32 {
        ((self.translation[0] - camera[0]).powi(2)
            + (self.translation[1] - camera[1]).powi(2)
            + (self.translation[2] - camera[2]).powi(2))
        .sqrt()
    }
}
