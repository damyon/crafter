use crate::button::Button;
use crate::command::Command;
use crate::command_queue::CommandQueue;
use crate::widget::Widget;

use glium::Frame;
use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;

pub struct UiContext {
    widgets: Vec<Box<dyn Widget>>,
    /// A queue of commands waiting to be processed.
    command_input: CommandQueue,
}

impl UiContext {
    /// Creates a ui context.
    pub const fn new() -> UiContext {
        UiContext {
            widgets: Vec::new(),
            command_input: CommandQueue::new(),
        }
    }

    /// Process the command queue.
    pub fn process_commands(&mut self) {
        let mut command_opt = self.command_input.next();

        while let Some(command) = command_opt {
            for widget in &mut self.widgets {
                widget.process_command(&command);
            }

            command_opt = self.command_input.next();
        }
    }

    pub fn create_default_ui(&mut self) {
        self.widgets.push(Box::new(Button::new((4, 4), (15, 8))));
    }

    /// Adds a widget to the UI context.
    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    /// Add a command to the queue of commands to process later.
    pub fn queue_command(&mut self, command: Command) {
        self.command_input.queue_command(command);
    }

    pub fn draw(&mut self, display: &Display<WindowSurface>, frame: &mut Frame) {
        for widget in &mut self.widgets {
            widget.draw(display, frame);
        }
    }
}
