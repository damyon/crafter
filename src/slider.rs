use crate::canvas::Canvas;
use crate::command::{Command, CommandType};
use glium::Frame;
use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;

pub struct Slider {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub background_color: [f32; 4],
    pub current_value: usize,
    pub range: (usize, usize),
    pub slider_index: u32,
}

impl Slider {
    pub fn new(
        position: (f32, f32),
        size: (f32, f32),
        background_color: [f32; 4],
        current_value: usize,
        range: (usize, usize),
        slider_index: u32,
    ) -> Self {
        Slider {
            position,
            size,
            background_color,
            current_value,
            range,
            slider_index,
        }
    }
}

use crate::widget::Widget;

impl Widget for Slider {
    fn draw(&mut self, display: &Display<WindowSurface>, frame: &mut Frame) {
        let mut canvas = Canvas::new(display, frame);

        let border_color = [0.1, 0.1, 0.1, 0.8];
        let color = self.background_color;
        let border = 0.01;
        canvas.draw_rectangle_with_border(self.position, self.size, color, border, border_color);

        // Draw the current position
        let vertical = (self.current_value as f32 / (self.range.1 - self.range.0) as f32
            * self.size.1)
            + self.position.1;

        canvas.draw_rectangle_with_border(
            (self.position.0, vertical - 0.02),
            (self.size.0, 0.04),
            [0.8, 0.8, 0.8, 0.8],
            0.01,
            [0.1, 0.1, 0.1, 0.8],
        );
    }

    fn process_command(&mut self, command: &Command) -> Vec<Command> {
        let mut translated_commands = Vec::new();
        // Process window event.
        match command.command_type {
            CommandType::MouseDown => {
                let x = f32::from_bits(command.data1);
                let y = f32::from_bits(command.data2);
                if x >= self.position.0
                    && x <= self.position.0 + self.size.0
                    && y >= self.position.1
                    && y <= self.position.1 + self.size.1
                {
                    let percentage = (y - self.position.1) / self.size.1;
                    let new_value =
                        percentage * (self.range.1 - self.range.0) as f32 + self.range.0 as f32;
                    self.current_value = new_value as usize;
                    translated_commands.push(Command {
                        command_type: CommandType::SliderMoved,
                        data1: self.slider_index,
                        data2: self.current_value as u32,
                    })
                }
            }
            CommandType::MouseMoved => {
                let x = f32::from_bits(command.data1);
                let y = f32::from_bits(command.data2);
                if x >= self.position.0
                    && x <= self.position.0 + self.size.0
                    && y >= self.position.1
                    && y <= self.position.1 + self.size.1
                {
                    let percentage = (y - self.position.1) / self.size.1;
                    let new_value =
                        percentage * (self.range.1 - self.range.0) as f32 + self.range.0 as f32;
                    self.current_value = new_value as usize;
                    translated_commands.push(Command {
                        command_type: CommandType::SliderMoved,
                        data1: self.slider_index,
                        data2: self.current_value as u32,
                    })
                }
            }
            CommandType::SetMaterialRed => {
                println!(
                    "We are setting the red in the slider {} {}",
                    self.slider_index, command.data2
                );
                if command.data2 == self.slider_index {
                    println!("We are setting the RED in THIS slider");
                    let percentage = f32::from_bits(command.data1);
                    let new_value =
                        percentage * (self.range.1 - self.range.0) as f32 + self.range.0 as f32;
                    self.current_value = new_value as usize;
                }
            }
            CommandType::SetMaterialGreen => {
                if command.data2 == self.slider_index {
                    let percentage = f32::from_bits(command.data1);
                    let new_value =
                        percentage * (self.range.1 - self.range.0) as f32 + self.range.0 as f32;
                    self.current_value = new_value as usize;
                }
            }
            CommandType::SetMaterialBlue => {
                if command.data2 == self.slider_index {
                    let percentage = f32::from_bits(command.data1);
                    let new_value =
                        percentage * (self.range.1 - self.range.0) as f32 + self.range.0 as f32;
                    self.current_value = new_value as usize;
                }
            }
            CommandType::SetMaterialAlpha => {
                if command.data2 == self.slider_index {
                    let percentage = f32::from_bits(command.data1);
                    let new_value =
                        percentage * (self.range.1 - self.range.0) as f32 + self.range.0 as f32;
                    self.current_value = new_value as usize;
                }
            }
            _ => (),
        }
        translated_commands
    }
}
