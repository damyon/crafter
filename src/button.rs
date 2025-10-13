use crate::command::Command;
use crate::image_vertex::ImageVertex;
use crate::vertex::Vertex;
use glium::Frame;
use glium::Surface;
use glium::backend::glutin::Display;
use glium::uniform;
use glutin::surface::WindowSurface;
use std::fs::File;
use std::io::BufReader;

pub struct ButtonState {
    pub name: String,
    pub icon_path: String,
}

pub struct Button {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub states: Vec<ButtonState>,
    pub current_state: String,
}

impl Button {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Self {
        Button {
            position,
            size,
            states: Vec::new(),
            current_state: String::new(),
        }
    }

    pub fn add_state(&mut self, name: String, icon_path: String) {
        let state_name = name.clone();
        if self.current_state.is_empty() {
            self.current_state = name;
        }
        self.states.push(ButtonState {
            name: state_name,
            icon_path,
        });
    }

    pub fn draw_rectangle(
        &self,
        display: &Display<WindowSurface>,
        frame: &mut Frame,
        position: (f32, f32),
        size: (f32, f32),
        color: [f32; 4],
    ) {
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

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
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
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        let uniforms = uniform! {
        u_color: color,
              };
        let params = glium::DrawParameters {
            line_width: Some(2.0),
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        frame
            .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
            .unwrap();
    }

    pub fn draw_image(
        &self,
        display: &Display<WindowSurface>,
        frame: &mut Frame,
        position: (f32, f32),
        size: (f32, f32),
        icon_path: &str,
    ) {
        let image_file = File::open(icon_path).unwrap();
        let buffered_reader = BufReader::new(image_file);
        let image = image::load(buffered_reader, image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        // 3. Create a glium texture
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap(); // Use SrgbTexture2d for correct color handling

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
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
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
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        let uniforms = uniform! {
            tex: &texture,
        };

        frame
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

use crate::widget::Widget;

impl Widget for Button {
    fn draw(&mut self, display: &Display<WindowSurface>, frame: &mut Frame) {
        let slices = 32;

        let mut angle: f32 = 0.0;
        let mut x: f32;
        let mut y: f32;
        let color = [0.1, 0.1, 0.1, 0.5];

        for _ in 0..slices {
            x = angle.cos() * 0.02;
            y = angle.sin() * 0.02;
            let pos_x = self.position.0 as f32 + x;
            let pos_y = self.position.1 as f32 + y;

            self.draw_rectangle(display, frame, (pos_x, pos_y), self.size, color);

            angle += 2.0 * std::f32::consts::PI / slices as f32;
        }
        for _ in 0..slices {
            x = angle.cos() * 0.01;
            y = angle.sin() * 0.01;
            let pos_x = self.position.0 as f32 + x;
            let pos_y = self.position.1 as f32 + y;

            self.draw_rectangle(
                display,
                frame,
                (pos_x, pos_y),
                self.size,
                [0.7, 0.6, 0.9, 1.0],
            );
            angle += 2.0 * std::f32::consts::PI / slices as f32;
        }

        if self.current_state.len() > 0 {
            let current = self
                .states
                .iter()
                .find(|state| state.name == self.current_state)
                .unwrap();

            self.draw_image(
                display,
                frame,
                (self.position.0, self.position.1),
                (self.size.0, self.size.1),
                current.icon_path.as_str(),
            );
        }
    }

    fn process_command(&mut self, command: &Command) {
        // Process window event.
    }
}
