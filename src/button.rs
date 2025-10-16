use crate::canvas::Canvas;
use crate::command::Command;
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
}

use crate::widget::Widget;

impl Widget for Button {
    fn draw(&mut self, display: &Display<WindowSurface>, frame: &mut Frame) {
        let mut canvas = Canvas::new(display, frame);
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

            canvas.draw_rectangle((pos_x, pos_y), self.size, color);

            angle += 2.0 * std::f32::consts::PI / slices as f32;
        }
        for _ in 0..slices {
            x = angle.cos() * 0.01;
            y = angle.sin() * 0.01;
            let pos_x = self.position.0 as f32 + x;
            let pos_y = self.position.1 as f32 + y;

            canvas.draw_rectangle((pos_x, pos_y), self.size, [0.7, 0.6, 0.9, 1.0]);
            angle += 2.0 * std::f32::consts::PI / slices as f32;
        }

        if self.current_state.len() > 0 {
            let current = self
                .states
                .iter()
                .find(|state| state.name == self.current_state)
                .unwrap();

            canvas.draw_image(
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
