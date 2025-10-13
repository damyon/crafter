use crate::command::Command;
use crate::vertex::Vertex;
use glium::Frame;
use glium::Surface;
use glium::backend::glutin::Display;
use glium::uniform;
use glutin::surface::WindowSurface;

pub struct Button {
    pub position: (u16, u16),
    pub size: (u16, u16),
}

impl Button {
    pub fn new(position: (u16, u16), size: (u16, u16)) -> Self {
        Button { position, size }
    }

    pub fn draw_rectangle(
        &self,
        display: &Display<WindowSurface>,
        frame: &mut Frame,
        position: (f32, f32),
        size: (u16, u16),
        color: [f32; 4],
    ) {
        // Draw the button at the specified position
        let vertex1 = Vertex {
            position: [position.0 as f32, position.1 as f32, 0.0],
            normal: [0.0, 0.0, 1.0],
        };
        let vertex2 = Vertex {
            position: [position.0 as f32, position.1 as f32 + size.1 as f32, 0.0],
            normal: [0.0, 0.0, 1.0],
        };
        let vertex3 = Vertex {
            position: [
                position.0 as f32 + size.0 as f32,
                position.1 as f32 + size.1 as f32,
                0.0,
            ],
            normal: [0.0, 0.0, 1.0],
        };
        let vertex4 = Vertex {
            position: [position.0 as f32 + size.0 as f32, position.1 as f32, 0.0],
            normal: [0.0, 0.0, 1.0],
        };
        let shape = vec![vertex1, vertex2, vertex3, vertex1, vertex3, vertex4];

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
            #version 140

            in vec3 position;

            void main() {
                const float scaling = 50.0;
                gl_Position = vec4(position / scaling - 1.0, 1.0);
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
            x = angle.cos() * 0.8;
            y = angle.sin() * 0.8;
            let pos_x = self.position.0 as f32 + x;
            let pos_y = self.position.1 as f32 + y;

            self.draw_rectangle(display, frame, (pos_x, pos_y), self.size, color);

            angle += 2.0 * std::f32::consts::PI / slices as f32;
        }
        for _ in 0..slices {
            x = angle.cos() * 0.5;
            y = angle.sin() * 0.5;
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
    }

    fn process_command(&mut self, command: &Command) {
        // Process window event.
    }
}
