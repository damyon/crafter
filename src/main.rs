use crate::command::Command;
use crate::command::CommandType;
use crate::graphics::Graphics;
use crate::scene::Scene;
use crate::ui_context::UiContext;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::winit::event::Event::{AboutToWait, WindowEvent};
use glium::winit::event::WindowEvent::{
    CloseRequested, CursorMoved, KeyboardInput, MouseInput, MouseWheel, RedrawRequested, Resized,
};
use glium::winit::event::{ElementState, MouseButton, MouseScrollDelta};
use glium::winit::event_loop::EventLoop;
use glium::winit::platform::scancode::PhysicalKeyExtScancode;
use std::time::Instant;
mod graphics;

mod button;
mod camera;
mod canvas;
mod command;
mod command_queue;
mod cube;
mod drawable;
mod grid;
mod image_vertex;
mod model;
mod mouse;
mod ocnode;
mod octree;
mod scene;
mod slider;
mod storage;
mod stored_octree;
mod ui_context;
mod vertex;
mod widget;

fn main() {
    let mut scene = Scene::new();

    scene.init();
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = EventLoop::builder().build().expect("event loop building");

    let (window, display) = SimpleWindowBuilder::new()
        .with_title("Crafter")
        .with_inner_size(800, 600)
        .build(&event_loop);
    let (width, height) = display.get_framebuffer_dimensions();

    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut window_width = width;
    let mut window_height = height;
    let mut graphics: Graphics = Graphics::new(width, height);
    graphics.setup_shaders(&display);

    let mut ui = UiContext::new();
    ui.create_default_ui();

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            match event {
                WindowEvent { event, .. } => match event {
                    // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                    CloseRequested => window_target.exit(),
                    Resized(window_size) => {
                        display.resize(window_size.into());
                        window_width = window_size.width;
                        window_height = window_size.height;
                        graphics = Graphics::new(window_size.width, window_size.height);
                        graphics.setup_shaders(&display);
                    }

                    RedrawRequested => {
                        scene.process_commands();
                        let translated_commands = ui.process_commands();
                        translated_commands.iter().for_each(|command| {
                            scene.queue_command(*command);
                        });

                        if scene.throttle() {
                            let start = Instant::now();
                            let mut frame = display.draw();
                            // By finishing the frame swap buffers and thereby make it visible on the window
                            scene.draw(&display, &mut frame, &mut graphics);
                            ui.draw(&display, &mut frame);
                            frame.finish().unwrap();
                            let end = Instant::now();
                            //       println!("Frame time: {:?}", end - start);
                        }
                    }
                    MouseInput {
                        device_id,
                        state,
                        button,
                    } => {
                        // Ignore the device ID for now.
                        _ = device_id;
                        match state {
                            ElementState::Pressed => match button {
                                MouseButton::Left => {
                                    // cursor to screen coordinates
                                    let screen_x =
                                        (cursor_x as f32 / window_width as f32) * 2.0 - 1.0;
                                    let screen_y =
                                        -((cursor_y as f32 / window_height as f32) * 2.0 - 1.0);

                                    let mouse_down = Command {
                                        command_type: CommandType::MouseDown,
                                        data1: screen_x.to_bits(),
                                        data2: screen_y.to_bits(),
                                    };
                                    scene.queue_command(mouse_down);
                                    ui.queue_command(mouse_down);
                                }
                                _ => {}
                            },
                            ElementState::Released => match button {
                                MouseButton::Left => {
                                    let mouse_up = Command {
                                        command_type: CommandType::MouseUp,
                                        data1: 1,
                                        data2: 1,
                                    };
                                    scene.queue_command(mouse_up);
                                    ui.queue_command(mouse_up);
                                }
                                _ => {}
                            },
                        }
                    }
                    CursorMoved {
                        device_id,
                        position,
                    } => {
                        // Ignore the device ID for now.
                        _ = device_id;
                        let mouse_moved = Command {
                            command_type: CommandType::MouseMoved,
                            data1: position.x as u32,
                            data2: position.y as u32,
                        };
                        cursor_x = position.x as u32;
                        cursor_y = position.y as u32;
                        scene.queue_command(mouse_moved);
                        ui.queue_command(mouse_moved);
                        scene.process_commands();
                    }
                    KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            let key_pressed = Command {
                                command_type: CommandType::KeyDown,
                                data1: event.physical_key.to_scancode().unwrap(),
                                data2: 0,
                            };
                            scene.queue_command(key_pressed);
                            ui.queue_command(key_pressed);
                            scene.process_commands();
                        }
                    }
                    MouseWheel { delta, .. } => match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            let mouse_wheel = Command {
                                command_type: CommandType::MouseScroll,
                                data1: x as u32,
                                data2: y as u32,
                            };
                            println!("Mouse wheel scrolled: x={}, y={}", x, y);
                            scene.queue_command(mouse_wheel);
                            ui.queue_command(mouse_wheel);
                            scene.process_commands();
                        }
                        _ => {}
                    },
                    _ => (),
                },
                AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            };
        })
        .unwrap();
}
