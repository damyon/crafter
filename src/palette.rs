use crate::canvas::Canvas;
use crate::command::{Command, CommandType};
use glium::Frame;
use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;

pub struct Palette {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: [f32; 4],
    pub noise: bool,
    pub fluid: bool,
    pub picker_icon_path: String,
    pub apply_icon_path: String,
    pub index: usize,
}

impl Palette {
    pub fn new(
        position: (f32, f32),
        size: (f32, f32),
        color: [f32; 4],
        noise: bool,
        fluid: bool,
        index: usize,
    ) -> Self {
        Palette {
            position,
            size,
            color,
            noise,
            fluid,
            picker_icon_path: String::from("resources/color-picker.png"),
            apply_icon_path: String::from("resources/color-apply.png"),
            index,
        }
    }
}

use crate::widget::Widget;

impl Widget for Palette {
    fn draw(&mut self, display: &Display<WindowSurface>, frame: &mut Frame) {
        let mut canvas = Canvas::new(display, frame);

        let border_color = [0.1, 0.1, 0.1, 0.8];
        let border = 0.01;
        canvas.draw_rectangle_with_border(
            self.position,
            (self.size.0, self.size.1),
            self.color,
            border,
            border_color,
        );
        canvas.draw_rectangle_with_border(
            self.position,
            (self.size.0 / 2.0, self.size.1 / 2.0),
            self.color,
            border / 2.0,
            border_color,
        );
        canvas.draw_image(
            (
                self.position.0 + border / 2.0,
                self.position.1 + border / 2.0,
            ),
            ((self.size.0 / 2.0) - border, (self.size.1 / 2.0) - border),
            self.picker_icon_path.as_str(),
        );
        canvas.draw_rectangle_with_border(
            (self.position.0 + self.size.0 / 2.0, self.position.1),
            (self.size.0 / 2.0, self.size.1 / 2.0),
            self.color,
            border / 2.0,
            border_color,
        );
        canvas.draw_image(
            (
                self.position.0 + border / 2.0 + self.size.0 / 2.0,
                self.position.1 + border / 2.0,
            ),
            (self.size.0 / 2.0 - border, self.size.1 / 2.0 - border),
            &self.apply_icon_path.as_str(),
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
                    && x <= self.position.0 + self.size.0 / 2.0
                    && y >= self.position.1
                    && y <= self.position.1 + self.size.1 / 2.0
                {
                    translated_commands.push(Command {
                        command_type: CommandType::PickMaterial,
                        data1: self.index as u32,
                        data2: self.index as u32,
                    })
                } else if x >= self.position.0 + self.size.0 / 2.0
                    && x <= self.position.0 + self.size.0
                    && y >= self.position.1
                    && y <= self.position.1 + self.size.1 / 2.0
                {
                    println!("Update current material color");
                    translated_commands.push(Command {
                        command_type: CommandType::UpdateCurrentMaterialRed,
                        data1: self.color[0].to_bits(),
                        data2: self.color[0].to_bits(),
                    });
                    translated_commands.push(Command {
                        command_type: CommandType::UpdateCurrentMaterialGreen,
                        data1: self.color[1].to_bits(),
                        data2: self.color[1].to_bits(),
                    });
                    translated_commands.push(Command {
                        command_type: CommandType::UpdateCurrentMaterialBlue,
                        data1: self.color[2].to_bits(),
                        data2: self.color[2].to_bits(),
                    });
                    translated_commands.push(Command {
                        command_type: CommandType::UpdateCurrentMaterialAlpha,
                        data1: self.color[3].to_bits(),
                        data2: self.color[3].to_bits(),
                    });
                }
            }
            CommandType::CurrentMaterialRed => {
                if command.data2 == self.index as u32 {
                    self.color[0] = f32::from_bits(command.data1);
                }
            }
            CommandType::CurrentMaterialGreen => {
                if command.data2 == self.index as u32 {
                    self.color[1] = f32::from_bits(command.data1);
                }
            }
            CommandType::CurrentMaterialBlue => {
                if command.data2 == self.index as u32 {
                    self.color[2] = f32::from_bits(command.data1);
                }
            }
            CommandType::CurrentMaterialAlpha => {
                if command.data2 == self.index as u32 {
                    self.color[3] = f32::from_bits(command.data1);
                }
            }
            CommandType::CurrentMaterialNoise => {
                if command.data2 == self.index as u32 {
                    self.noise = command.data1 == 1;
                }
            }
            CommandType::CurrentMaterialFluid => {
                if command.data2 == self.index as u32 {
                    self.fluid = command.data1 == 1;
                }
            }
            _ => (),
        }
        translated_commands
    }
}
