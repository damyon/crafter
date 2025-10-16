use crate::image_vertex::ImageVertex;
use crate::vertex::Vertex;
use glium::Frame;
use glium::Surface;
use glium::backend::glutin::Display;
use glium::uniform;
use glutin::surface::WindowSurface;
use std::fs::File;
use std::io::BufReader;

pub struct Canvas<'a> {
    display: &'a Display<WindowSurface>,
    frame: &'a mut Frame,
}

impl<'a> Canvas<'a> {
    pub fn new(display: &'a Display<WindowSurface>, frame: &'a mut Frame) -> Self {
        Canvas { display, frame }
    }

    pub fn draw_rectangle(&mut self, position: (f32, f32), size: (f32, f32), color: [f32; 4]) {
        // Draw the rect at the specified position
        let vertex1 = Vertex {
            position: [position.0, position.1, 0.0],
            normal: [0.0, 0.0, 1.0],
        };
        let vertex2 = Vertex {
            position: [position.0, position.1 + size.1, 0.0],
            normal: [0.0, 0.0, 1.0],
        };
        let vertex3 = Vertex {
            position: [position.0 + size.0, position.1 + size.1, 0.0],
            normal: [0.0, 0.0, 1.0],
        };
        let vertex4 = Vertex {
            position: [position.0 + size.0, position.1, 0.0],
            normal: [0.0, 0.0, 1.0],
        };
        let shape = vec![vertex1, vertex2, vertex3, vertex1, vertex3, vertex4];

        let vertex_buffer = glium::VertexBuffer::new(self.display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            void main() {
                gl_Position = vec4(position, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140
            uniform vec4 u_color;
            out vec4 color;
            void main() {
                color = u_color;
            }
        "#;
        let program =
            glium::Program::from_source(self.display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        let uniforms = uniform! {
        u_color: color,
              };
        let params = glium::DrawParameters {
            line_width: Some(2.0),
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        self.frame
            .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
            .unwrap();
    }

    pub fn draw_circle(
        &mut self,
        position: (f32, f32),
        radius: f32,
        color: [f32; 4],
        start_angle: f32,
        end_angle: f32,
    ) {
        let slices = 8;
        let mut vertices: Vec<Vertex> = Vec::with_capacity(slices * 3);

        let mut angle = start_angle;
        let pie_angle = (end_angle - start_angle) / (slices as f32);
        let mut end_angle = angle + pie_angle;
        let mut x: f32;
        let mut y: f32;
        let mut x2: f32;
        let mut y2: f32;

        for _ in 0..slices {
            x = angle.cos() * radius;
            y = angle.sin() * radius;
            x2 = end_angle.cos() * radius;
            y2 = end_angle.sin() * radius;

            vertices.push(Vertex {
                position: [position.0, position.1, 0.0],
                normal: [0.0, 0.0, 1.0],
            });
            vertices.push(Vertex {
                position: [position.0 + x, position.1 + y, 0.0],
                normal: [0.0, 0.0, 1.0],
            });
            vertices.push(Vertex {
                position: [position.0 + x2, position.1 + y2, 0.0],
                normal: [0.0, 0.0, 1.0],
            });
            angle += pie_angle;
            end_angle = angle + pie_angle;
        }

        let vertex_buffer = glium::VertexBuffer::new(self.display, &vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            void main() {
                gl_Position = vec4(position, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140
            uniform vec4 u_color;
            out vec4 color;
            void main() {
                color = u_color;
            }
        "#;
        let program =
            glium::Program::from_source(self.display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        let uniforms = uniform! {
            u_color: color,
        };
        let params = glium::DrawParameters {
            line_width: Some(2.0),
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        self.frame
            .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
            .unwrap();
    }

    pub fn draw_rectangle_with_border(
        &mut self,
        position: (f32, f32),
        size: (f32, f32),
        color: [f32; 4],
        border: f32,
        border_color: [f32; 4],
    ) {
        // Draw the rect at the specified position
        let inset_position = (position.0 + border, position.1 + border);
        let inset_size = (size.0 - (2.0 * border), size.1 - (2.0 * border));
        self.draw_rectangle(inset_position, inset_size, color);
        let left_position = (position.0, position.1 + border);
        let left_size = (border, size.1 - (2.0 * border));
        let right_position = (
            position.0 + border + size.0 - (2.0 * border),
            position.1 + border,
        );
        let right_size = (border, size.1 - (2.0 * border));
        let top_position = (
            position.0 + border,
            position.1 + border + size.1 - (2.0 * border),
        );
        let top_size = (size.0 - (2.0 * border), border);
        let bottom_position = (position.0 + border, position.1);
        let bottom_size = (size.0 - (2.0 * border), border);
        self.draw_rectangle(left_position, left_size, border_color);
        self.draw_rectangle(right_position, right_size, border_color);
        self.draw_rectangle(top_position, top_size, border_color);
        self.draw_rectangle(bottom_position, bottom_size, border_color);

        self.draw_circle(
            inset_position,
            border,
            border_color,
            std::f32::consts::PI,
            1.5 * std::f32::consts::PI,
        );
        self.draw_circle(
            (inset_position.0 + inset_size.0, inset_position.1),
            border,
            border_color,
            1.5 * std::f32::consts::PI,
            2.0 * std::f32::consts::PI,
        );
        self.draw_circle(
            (
                inset_position.0 + inset_size.0,
                inset_position.1 + inset_size.1,
            ),
            border,
            border_color,
            0.0,
            0.5 * std::f32::consts::PI,
        );
        self.draw_circle(
            (inset_position.0, inset_position.1 + inset_size.1),
            border,
            border_color,
            0.5 * std::f32::consts::PI,
            1.0 * std::f32::consts::PI,
        );
    }

    pub fn draw_image(&mut self, position: (f32, f32), size: (f32, f32), icon_path: &str) {
        let image_file = File::open(icon_path).unwrap();
        let buffered_reader = BufReader::new(image_file);
        let image = image::load(buffered_reader, image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        // 3. Create a glium texture
        let texture = glium::texture::SrgbTexture2d::new(self.display, image).unwrap(); // Use SrgbTexture2d for correct color handling

        // 4. Define the quad vertices (full screen)
        let shape = vec![
            ImageVertex {
                position: [position.0, position.1],
                tex_coords: [0.0, 0.0],
            },
            ImageVertex {
                position: [position.0 + size.0, position.1],
                tex_coords: [1.0, 0.0],
            },
            ImageVertex {
                position: [position.0 + size.0, position.1 + size.1],
                tex_coords: [1.0, 1.0],
            },
            ImageVertex {
                position: [position.0 + size.0, position.1 + size.1],
                tex_coords: [1.0, 1.0],
            },
            ImageVertex {
                position: [position.0, position.1 + size.1],
                tex_coords: [0.0, 1.0],
            },
            ImageVertex {
                position: [position.0, position.1],
                tex_coords: [0.0, 0.0],
            },
        ];
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let vertex_buffer = glium::VertexBuffer::new(self.display, &shape).unwrap();
        let vertex_shader_src = r#"
                #version 140

                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;

                void main() {
                    v_tex_coords = tex_coords;

                    gl_Position = vec4(position, 0.0, 1.0);
                }
            "#;
        let fragment_shader_src = r#"
                #version 140

                in vec2 v_tex_coords;
                out vec4 color;

                uniform sampler2D tex;

                void main() {
                    color = texture(tex, v_tex_coords);
                }
            "#;
        let program =
            glium::Program::from_source(self.display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        let uniforms = uniform! {
            tex: &texture,
        };
        let params = glium::DrawParameters {
            line_width: Some(2.0),
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        self.frame
            .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
            .unwrap();
    }
}
