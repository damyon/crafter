use crate::canvas::Canvas;
use crate::command::{Command, CommandType};
use glium::Frame;
use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;

pub struct ButtonState {
    pub name: String,
    pub icon_path: String,
}

pub struct Button {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub states: Vec<ButtonState>,
    pub current_state: usize,
}

impl Button {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Self {
        Button {
            position,
            size,
            states: Vec::new(),
            current_state: 0,
        }
    }

    pub fn add_state(&mut self, name: String, icon_path: String) {
        let state_name = name.clone();
        if self.states.len() == 0 {
            self.current_state = 0;
        }
        self.states.push(ButtonState {
            name: state_name,
            icon_path,
        });
    }
}

use crate::widget::Widget;

impl Widget for Button {
    fn draw(&mut self, display: &Display<WindowSurface>, frame: &mut Frame) {
        let mut canvas = Canvas::new(display, frame);

        let border_color = [0.1, 0.1, 0.1, 0.8];
        let color = [0.6, 0.6, 0.6, 1.0];
        let border = 0.01;
        canvas.draw_rectangle_with_border(self.position, self.size, color, border, border_color);
        if self.states.len() > 0 {
            let current = self.states.get(self.current_state).unwrap();

            canvas.draw_image(
                (
                    self.position.0 + border * 2.0,
                    self.position.1 + border * 2.0,
                ),
                (self.size.0 - border * 4.0, self.size.1 - border * 4.0),
                current.icon_path.as_str(),
            );
        }
    }

    fn process_command(&mut self, command: &Command) {
        // Process window event.
        match command.command_type {
            CommandType::MouseDown => {
                let x = f32::from_bits(command.data1);
                let y = f32::from_bits(command.data2);
                println!("Mouse down at ({}, {})", x, y);
                if x >= self.position.0
                    && x <= self.position.0 + self.size.0
                    && y >= self.position.1
                    && y <= self.position.1 + self.size.1
                {
                    self.current_state = (self.current_state + 1) % self.states.len();
                }
            }
            _ => {}
        }
    }
}
