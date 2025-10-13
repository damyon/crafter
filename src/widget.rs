use crate::command::Command;

use glium::Frame;
use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;

pub trait Widget {
    fn draw(&mut self, display: &Display<WindowSurface>, frame: &mut Frame);
    fn process_command(&mut self, command: &Command);
}
