use crate::canvas::Canvas;
use crate::command::{Command, CommandType};
use glium::Frame;
use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;

pub struct Swatch {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub current_color: [f32; 4],
}

impl Swatch {
    pub fn new(position: (f32, f32), size: (f32, f32), current_color: [f32; 4]) -> Self {
        Swatch {
            position,
            size,
            current_color,
        }
    }
}

use crate::widget::Widget;

impl Widget for Swatch {
    fn draw(&mut self, display: &Display<WindowSurface>, frame: &mut Frame) {
        let mut canvas = Canvas::new(display, frame);

        let border_color = [0.1, 0.1, 0.1, 0.8];
        let border = 0.01;
        canvas.draw_rectangle_with_border(
            self.position,
            self.size,
            self.current_color,
            border,
            border_color,
        );
    }

    fn process_command(&mut self, command: &Command) -> Vec<Command> {
        let mut translated_commands = Vec::new();

        // Process window event.
        match command.command_type {
            CommandType::SetMaterialRed => {
                let red = f32::from_bits(command.data1);
                self.current_color[0] = red;
            }
            CommandType::SetMaterialGreen => {
                let green = f32::from_bits(command.data1);
                self.current_color[1] = green;
            }
            CommandType::SetMaterialBlue => {
                let blue = f32::from_bits(command.data1);
                self.current_color[2] = blue;
            }
            CommandType::SetMaterialAlpha => {
                let alpha = f32::from_bits(command.data1);
                self.current_color[3] = alpha;
            }
            _ => (),
        }
        translated_commands
    }
}
