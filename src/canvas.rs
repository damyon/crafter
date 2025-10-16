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
        // Draw the button at the specified position
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

        self.frame
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
