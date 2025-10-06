use crate::command::Command;
use crate::command::CommandType;
use crate::graphics::Graphics;
use crate::scene::Scene;
use glium::Surface;
use glium::winit::event::ElementState;
use glium::winit::event::MouseButton;
use glium::winit::event_loop::ControlFlow;
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
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Crafter")
        .with_inner_size(800, 600)
        .build(&event_loop);

    // Start rendering by creating a new frame
    let mut frame = display.draw();
    // Which we fill with an opaque blue color
    frame.clear_color(0.3, 0.0, 1.0, 1.0);
    // By finishing the frame swap buffers and thereby make it visible on the window
    frame.finish().unwrap();

    // Now we wait until the program is closed
    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            window_target.set_control_flow(ControlFlow::Poll);

            match event {
                glium::winit::event::Event::WindowEvent { event, .. } => match event {
                    // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                    glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    glium::winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    glium::winit::event::WindowEvent::RedrawRequested => {
                        let mut frame = display.draw();
                        let (width, height) = display.get_framebuffer_dimensions();
                        let mut graphics: Graphics =
                            Graphics::new(&display, &mut frame, width, height);
                        graphics.setup_shaders();
                        // By finishing the frame swap buffers and thereby make it visible on the window
                        scene.draw(&mut graphics);
                        frame.finish().unwrap();

                        window.request_redraw();
                    }
                    glium::winit::event::WindowEvent::MouseInput {
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
                                    window.request_redraw();
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
                                    window.request_redraw();
                                }
                                _ => {}
                            },
                        }
                    }
                    glium::winit::event::WindowEvent::CursorMoved {
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
                        window.request_redraw();
                    }
                    _ => (),
                },
                glium::winit::event::Event::AboutToWait => {
                    scene.process_commands();
                    window.request_redraw();
                }
                _ => (),
            };
        })
        .unwrap();
}
