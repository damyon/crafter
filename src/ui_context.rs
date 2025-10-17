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
    pub fn process_commands(&mut self) -> Vec<Command> {
        let mut command_opt = self.command_input.next();
        let mut translated_commands = Vec::<Command>::new();

        while let Some(command) = command_opt {
            for widget in &mut self.widgets {
                let translated_command_opt = widget.process_command(&command);
                if let Some(translated_command) = translated_command_opt {
                    translated_commands.push(translated_command);
                }
            }

            command_opt = self.command_input.next();
        }

        translated_commands
    }

    pub fn create_default_ui(&mut self) {
        let mut button = Button::new((-0.8, -0.95), (0.1, 0.1), 34);
        button.add_state(
            String::from("Hide Grid"),
            String::from("resources/show-grid.png"),
        );
        button.add_state(
            String::from("Show Grid"),
            String::from("resources/hide-grid.png"),
        );

        self.add_widget(Box::new(button));

        let mut button = Button::new((-0.69, -0.95), (0.1, 0.1), 20);
        button.add_state(
            String::from("Sphere Shape"),
            String::from("resources/shape-sphere.png"),
        );
        button.add_state(
            String::from("Cube Shape"),
            String::from("resources/shape-cube.png"),
        );
        button.add_state(
            String::from("Square XZ Shape"),
            String::from("resources/shape-square-xz.png"),
        );
        button.add_state(
            String::from("Square XY Shape"),
            String::from("resources/shape-square-xy.png"),
        );
        button.add_state(
            String::from("Square YZ Shape"),
            String::from("resources/shape-square-yz.png"),
        );
        button.add_state(
            String::from("Circle XZ Shape"),
            String::from("resources/shape-circle-xz.png"),
        );
        button.add_state(
            String::from("Circle XY Shape"),
            String::from("resources/shape-circle-xy.png"),
        );
        button.add_state(
            String::from("Circle YZ Shape"),
            String::from("resources/shape-circle-yz.png"),
        );

        self.add_widget(Box::new(button));
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
