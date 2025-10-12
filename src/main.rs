use crate::command::Command;
use crate::command::CommandType;
use crate::graphics::Graphics;
use crate::scene::Scene;
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

mod camera;
mod command;
mod command_queue;
mod cube;
mod drawable;
mod grid;
mod model;
mod mouse;
mod ocnode;
mod octree;
mod scene;
mod storage;
mod stored_octree;
mod vertex;

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

    let mut graphics: Graphics = Graphics::new(width, height);
    graphics.setup_shaders(&display);

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            match event {
                WindowEvent { event, .. } => match event {
                    // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                    CloseRequested => window_target.exit(),
                    Resized(window_size) => {
                        display.resize(window_size.into());
                        graphics = Graphics::new(window_size.width, window_size.height);
                        graphics.setup_shaders(&display);
                    }

                    RedrawRequested => {
                        scene.process_commands();
                        if scene.throttle() {
                            let start = Instant::now();
                            let mut frame = display.draw();
                            // By finishing the frame swap buffers and thereby make it visible on the window
                            scene.draw(&display, &mut frame, &mut graphics);
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
                                    let mouse_down = Command {
                                        command_type: CommandType::MouseDown,
                                        data1: 1,
                                        data2: 1,
                                    };
                                    scene.queue_command(mouse_down);
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
                        scene.queue_command(mouse_moved);
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
