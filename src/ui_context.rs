use crate::button::Button;
use crate::command::Command;
use crate::command_queue::CommandQueue;
use crate::palette::Palette;
use crate::slider::Slider;
use crate::swatch::Swatch;
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
                translated_commands.extend(widget.process_command(&command));
            }

            command_opt = self.command_input.next();
        }

        translated_commands
    }

    pub fn create_default_ui(&mut self) {
        let mut button = Button::new((-0.96, -0.95), (0.1, 0.1), 1);
        button.add_state(String::from("resources/file-open.png"));

        self.add_widget(Box::new(button));

        let mut button = Button::new((-0.85, -0.95), (0.1, 0.1), 2);
        button.add_state(String::from("resources/file-save.png"));

        self.add_widget(Box::new(button));

        let mut button = Button::new((-0.74, -0.95), (0.1, 0.1), 34);
        button.add_state(String::from("resources/show-grid.png"));
        button.add_state(String::from("resources/hide-grid.png"));

        self.add_widget(Box::new(button));

        let mut button = Button::new((-0.63, -0.95), (0.1, 0.1), 20);
        button.add_state(String::from("resources/shape-sphere.png"));
        button.add_state(String::from("resources/shape-cube.png"));
        button.add_state(String::from("resources/shape-square-xz.png"));
        button.add_state(String::from("resources/shape-square-xy.png"));
        button.add_state(String::from("resources/shape-square-yz.png"));
        button.add_state(String::from("resources/shape-circle-xz.png"));
        button.add_state(String::from("resources/shape-circle-xy.png"));
        button.add_state(String::from("resources/shape-circle-yz.png"));

        self.add_widget(Box::new(button));

        let mut button = Button::new((-0.52, -0.95), (0.1, 0.1), 33);
        button.add_state(String::from("resources/material-solid.png"));
        button.add_state(String::from("resources/material-fluid.png"));

        self.add_widget(Box::new(button));

        let mut button = Button::new((-0.41, -0.95), (0.1, 0.1), 49);
        button.add_state(String::from("resources/shader-solid.png"));
        button.add_state(String::from("resources/shader-noise.png"));

        self.add_widget(Box::new(button));

        // Red slider
        let slider = Slider::new(
            (-0.3, -0.95),
            (0.05, 0.3),
            [1.0, 0.0, 0.0, 1.0],
            204,
            (0, 255),
            0,
        );

        self.add_widget(Box::new(slider));

        // Green slider
        let slider = Slider::new(
            (-0.25, -0.95),
            (0.05, 0.3),
            [0.0, 1.0, 0.0, 1.0],
            204,
            (0, 255),
            1,
        );

        self.add_widget(Box::new(slider));

        // Blue slider
        let slider = Slider::new(
            (-0.2, -0.95),
            (0.05, 0.3),
            [0.0, 0.0, 1.0, 1.0],
            204,
            (0, 255),
            2,
        );
        self.add_widget(Box::new(slider));
        // Alpha slider
        let slider = Slider::new(
            (-0.15, -0.95),
            (0.05, 0.3),
            [0.5, 0.5, 0.5, 1.0],
            255,
            (0, 255),
            3,
        );

        self.add_widget(Box::new(slider));

        let swatch = Swatch::new((-0.09, -0.95), (0.1, 0.1), [0.8, 0.8, 0.8, 1.0]);

        self.add_widget(Box::new(swatch));

        let palette = Palette::new(
            (0.02, -0.95),
            (0.1, 0.1),
            [0.8, 0.8, 0.8, 1.0],
            false,
            false,
            0,
        );

        self.add_widget(Box::new(palette));
        let palette = Palette::new(
            (0.13, -0.95),
            (0.1, 0.1),
            [0.8, 0.8, 0.8, 1.0],
            false,
            false,
            1,
        );

        self.add_widget(Box::new(palette));
        let palette = Palette::new(
            (0.24, -0.95),
            (0.1, 0.1),
            [0.8, 0.8, 0.8, 1.0],
            false,
            false,
            2,
        );

        self.add_widget(Box::new(palette));
        let palette = Palette::new(
            (0.35, -0.95),
            (0.1, 0.1),
            [0.8, 0.8, 0.8, 1.0],
            false,
            false,
            3,
        );

        self.add_widget(Box::new(palette));
        let palette = Palette::new(
            (0.46, -0.95),
            (0.1, 0.1),
            [0.8, 0.8, 0.8, 1.0],
            false,
            false,
            4,
        );

        self.add_widget(Box::new(palette));
        let palette = Palette::new(
            (0.57, -0.95),
            (0.1, 0.1),
            [0.8, 0.8, 0.8, 1.0],
            false,
            false,
            5,
        );

        self.add_widget(Box::new(palette));
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
