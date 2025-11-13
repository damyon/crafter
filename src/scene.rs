use crate::command::{Command, CommandType};
use crate::command_queue::CommandQueue;
use crate::drawable::Drawable;
use crate::graphics::Graphics;
use crate::grid::Grid;
use crate::material::Material;
use crate::model::Model;
use crate::mouse::Mouse;
use crate::ocnode::Ocnode;
use crate::vertex::Vertex;
use crate::{camera::Camera, cube::Cube};
use glium::Frame;
use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;
use nalgebra::*;
use rfd::FileDialog;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Simple list of supported selection shapes.
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SelectionShape {
    Sphere,
    Cube,
    SquareXZ,
    SquareXY,
    SquareYZ,
    CircleXZ,
    CircleXY,
    CircleYZ,
}

/// This represents the data and the links to input/output required to render the scene.
pub struct Scene {
    /// The current camera.
    pub camera: Camera,
    /// The current light.
    pub light: Camera,
    /// The mouse info.
    mouse: Mouse,
    /// A queue of commands waiting to be processed.
    command_input: CommandQueue,
    /// A cube that is used to draw the selection shape.
    selection_cube: Cube,
    /// We could show more, but only the flat grid is enough.
    grid_xz: Grid,
    /// This is the octree of voxels.
    model: Model,
    /// Where is the selection.
    selection_position: [i32; 3],
    /// What is the size of the selection.
    selection_radius: u32,
    /// What shape is the selection.
    selection_shape: SelectionShape,
    /// What colour will we fill if the selection is toggled.
    material_color: [f32; 4],
    /// Are we currently drawing a frame?
    drawing: bool,
    /// Should we skip the next frame?
    throttle: u32,
    /// Timestamp from last render.
    last_draw: Option<Instant>,
    /// Are we loading from browser?
    loading: bool,
    /// Is the material fluid?
    fluid: bool,
    /// Is the material noisy?
    noise: bool,
    /// Will the frame match the last rendered frame?
    dirty: bool,
    /// Approximation of time
    elapsed: f32,
    /// Render the grid
    grid_visible: bool,
    /// Speed of re-drawing when screen is idle.
    target_fps: u32,
    /// Only recalculate the drawables cache if the scene has changed.
    drawables_cache: Vec<Cube>,
    /// Clear the drawables cache.
    invalidate_drawables_cache: bool,
    /// Start time of the scene.
    start_time: Option<Instant>,
    /// Hashmap to store rendered vertices for each material.
    render_cache: Option<HashMap<Material, Vec<Vertex>>>,
    /// Invalidate the render cache.
    invalidate_render_cache: bool,
    /// Invalidate a single material from the render cache.
    invalidate_render_material: Option<Material>,
    /// Invalidate the selection vertices.
    invalidate_selection_render_cache: bool,
    /// Vec of selection vertices.
    selection_vertices_cache: Option<Vec<Vertex>>,
}

impl Scene {
    pub const fn new() -> Scene {
        Scene {
            camera: Camera::new(),
            light: Camera::new(),
            mouse: Mouse::new(),
            command_input: CommandQueue::new(),
            selection_cube: Cube::new(),
            grid_xz: Grid::new(),
            model: Model::new(),
            selection_position: [0, 0, 0],
            selection_radius: 1,
            selection_shape: SelectionShape::Sphere,
            material_color: [0.8, 0.8, 0.8, 1.0],
            drawing: false,
            throttle: 10,
            loading: true,
            fluid: false,
            noise: false,
            dirty: true,
            elapsed: 0.0,
            last_draw: None,
            grid_visible: true,
            target_fps: 30,
            drawables_cache: Vec::new(),
            invalidate_drawables_cache: false,
            start_time: None,
            render_cache: None,
            invalidate_render_cache: false,
            invalidate_render_material: None,
            invalidate_selection_render_cache: false,
            selection_vertices_cache: None,
        }
    }

    fn select_file_to_open(&mut self) {
        let file = FileDialog::new()
            .set_directory(".") // Optional: set the starting directory
            .add_filter("Scene", &["scn"])
            .pick_file();

        if let Some(path) = file {
            println!("The user picked: {:?}", path);
            let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];

            self.model
                .load(path.as_path().to_str().unwrap(), camera_eye);
            self.invalidate_drawables_cache = true;

            self.model.recalculate_occlusion();
            self.invalidate_render_cache = true;
        } else {
            println!("The user canceled the operation.");
        }
    }

    fn select_file_to_save(&mut self) {
        let file = FileDialog::new()
            .set_directory(".")
            .add_filter("Scene", &["scn"])
            .save_file();

        if let Some(path) = file {
            println!("The user picked: {:?}", path);

            self.model.save(path.as_path().to_str().unwrap());
        } else {
            println!("The user canceled the operation.");
        }
    }

    /// Helper function to rotate a point around an axis.

    /// Add a command to the queue of commands to process later.
    pub fn queue_command(&mut self, command: Command) {
        self.dirty = true;

        self.command_input.queue_command(command);
    }

    /// Process a mouse down event.
    pub fn handle_mouse_down(&mut self) {
        if self.mouse.last_position[0] > -0.2
            && self.mouse.last_position[0] < 0.2
            && self.mouse.last_position[1] > -0.2
            && self.mouse.last_position[1] < 0.2
        {
            self.mouse.is_pressed = true;
        }
    }

    /// Process a mouse up event.
    pub fn handle_mouse_up(&mut self) {
        self.mouse.is_pressed = false;
    }

    /// Process a mouse moved event.
    pub fn handle_mouse_moved(&mut self, command: &Command) {
        let x = f32::from_bits(command.data1);
        let y = f32::from_bits(command.data2);
        let current_position = Point2::new(x, y);

        if self.mouse.is_pressed && (y > -0.6) {
            let position_diff = Point2::new(
                current_position.x - self.mouse.last_position.x,
                current_position.y - self.mouse.last_position.y,
            );
            let current_camera_eye = self.camera.eye;
            let current_camera_target = self.camera.target;
            let current_camera_direction = current_camera_target - current_camera_eye;

            let blunting = 0.8;

            let pitch = position_diff.x * blunting;
            let yaw = position_diff.y * blunting;

            let rotation = Rotation3::from_euler_angles(0.0, pitch, yaw);
            let new_camera_direction = rotation * current_camera_direction;
            self.camera.target = Point3::new(
                current_camera_eye.x + new_camera_direction.x,
                current_camera_eye.y + new_camera_direction.y,
                current_camera_eye.z + new_camera_direction.z,
            );
        }
        self.mouse.last_position = current_position;
    }

    /// The key was pressed to move up.
    pub fn handle_move_up(&mut self) {
        self.camera.eye = Point3::new(
            self.camera.eye.x,
            self.camera.eye.y + 0.1_f32,
            self.camera.eye.z,
        );
        self.camera.target = Point3::new(
            self.camera.target.x,
            self.camera.target.y + 0.1_f32,
            self.camera.target.z,
        );

        let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
        self.model.optimize(camera_eye);
        self.invalidate_drawables_cache = true;
    }

    /// The key was pressed to move down.
    pub fn handle_move_down(&mut self) {
        self.camera.eye = Point3::new(
            self.camera.eye.x,
            self.camera.eye.y - 0.1_f32,
            self.camera.eye.z,
        );
        self.camera.target = Point3::new(
            self.camera.target.x,
            self.camera.target.y - 0.1_f32,
            self.camera.target.z,
        );
        let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
        self.model.optimize(camera_eye);
        self.invalidate_drawables_cache = true;
    }

    /// The key was pressed to move left.
    pub fn handle_move_left(&mut self) {
        let diff = self.camera.target - self.camera.eye;
        let blunting = 10.0;
        //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
        let projection = Vector3::new(diff.z, 0.0, -diff.x) / blunting;

        self.camera.eye += projection;
        self.camera.target += projection;
        let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
        self.model.optimize(camera_eye);
        self.invalidate_drawables_cache = true;
    }

    /// The key was pressed to move right.
    pub fn handle_move_right(&mut self) {
        let diff = self.camera.target - self.camera.eye;
        let blunting = 10.0;
        //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
        let projection = Vector3::new(diff.z, 0.0, -diff.x) / blunting;

        self.camera.eye -= projection;
        self.camera.target -= projection;
        let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
        self.model.optimize(camera_eye);
        self.invalidate_drawables_cache = true;
    }

    /// The key was pressed to move forward.
    pub fn handle_move_forward(&mut self) {
        let diff = self.camera.target - self.camera.eye;
        let blunting = 10.0;
        let projection = Vector3::new(diff.x, diff.y, diff.z) / blunting;

        self.camera.eye += projection;
        self.camera.target += projection;
        let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
        self.model.optimize(camera_eye);
        self.invalidate_drawables_cache = true;
    }

    /// The key was pressed to move backwards.
    pub fn handle_move_backward(&mut self) {
        let diff = self.camera.target - self.camera.eye;
        let blunting = 10.0;
        let projection = Vector3::new(-diff.x, -diff.y, -diff.z) / blunting;

        self.camera.eye += projection;
        self.camera.target += projection;
        let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
        self.model.optimize(camera_eye);
        self.invalidate_drawables_cache = true;
    }

    /// The key was pressed to toggle the state of the current selection.
    pub fn handle_toggle_voxel(&mut self) {
        log::info!("Start toggling voxel");
        let selections = Self::selection_voxels(
            &self.selection_position,
            self.selection_radius as i32,
            self.selection_shape,
        );

        log::info!("Checking if all voxels are active");
        let value: bool = self.model.all_voxels_active(&selections);
        log::info!("Checking done: {}", value);

        let count = selections.len();
        let fluid = self.fluid;
        let noise = self.noise;
        if value {
            log::info!("Toggle all voxels active: FALSE {count} {fluid} {noise}");
        } else {
            log::info!("Toggle all voxels active: TRUE {count} {fluid} {noise}");
        }
        let color = [
            (self.material_color[0]).clamp(0.0, 1.0),
            (self.material_color[1]).clamp(0.0, 1.0),
            (self.material_color[2]).clamp(0.0, 1.0),
            (self.material_color[3]).clamp(0.0, 1.0),
        ];
        let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
        let fluid = if self.fluid { 1 } else { 0 };
        let noise = if self.noise { 1 } else { 0 };
        println!("Scene toggle voxels");
        self.model
            .toggle_voxels(selections, !value, color, camera_eye, fluid, noise);
        println!("Scene toggle voxels done");
        self.invalidate_drawables_cache = true;
        let selections = Self::selection_voxels(
            &self.selection_position,
            self.selection_radius as i32,
            self.selection_shape,
        );
        println!("Scene recalculate_occlusion_for_selections");
        self.model.recalculate_occlusion_for_selections(selections);
        println!("Scene recalculate_occlusion_for_selections DONE");
        self.invalidate_render_cache = true;
    }

    /// Save the scene to the browser.

    /// Move the selection shape left.
    pub fn handle_move_selection_left(&mut self) {
        self.selection_cube.translate([-1.0, 0.0, 0.0]);
        self.selection_position[0] -= 1;
        self.invalidate_selection_render_cache = true;
    }

    /// Move the selection shape right.
    pub fn handle_move_selection_right(&mut self) {
        self.selection_cube.translate([1.0, 0.0, 0.0]);
        self.selection_position[0] += 1;
        self.invalidate_selection_render_cache = true;
    }

    /// Move the selection shape forward.
    pub fn handle_move_selection_forward(&mut self) {
        self.selection_cube.translate([0.0, 0.0, 1.0]);
        self.selection_position[2] += 1;
        self.invalidate_selection_render_cache = true;
    }

    /// Move the selection shape backward.
    pub fn handle_move_selection_backward(&mut self) {
        self.selection_cube.translate([0.0, 0.0, -1.0]);
        self.selection_position[2] -= 1;
        self.invalidate_selection_render_cache = true;
    }

    /// Move the selection shape up.
    pub fn handle_move_selection_up(&mut self) {
        self.selection_cube.translate([0.0, 1.0, 0.0]);
        self.selection_position[1] += 1;
        self.invalidate_selection_render_cache = true;
    }

    /// Move the selection shape down.
    pub fn handle_move_selection_down(&mut self) {
        self.selection_cube.translate([0.0, -1.0, 0.0]);
        self.selection_position[1] -= 1;
        self.invalidate_selection_render_cache = true;
    }

    /// Hide or show the selection shape.
    pub fn handle_toggle_selection_shape(&mut self) {
        self.selection_shape = if self.selection_shape == SelectionShape::Sphere {
            SelectionShape::Cube
        } else if self.selection_shape == SelectionShape::Cube {
            SelectionShape::SquareXZ
        } else if self.selection_shape == SelectionShape::SquareXZ {
            SelectionShape::SquareXY
        } else if self.selection_shape == SelectionShape::SquareXY {
            SelectionShape::SquareYZ
        } else if self.selection_shape == SelectionShape::SquareYZ {
            SelectionShape::CircleXZ
        } else if self.selection_shape == SelectionShape::CircleXZ {
            SelectionShape::CircleXY
        } else if self.selection_shape == SelectionShape::CircleXY {
            SelectionShape::CircleYZ
        } else {
            SelectionShape::Sphere
        };
        self.invalidate_selection_render_cache = true;
    }

    pub fn handle_slider_moved(&mut self, command: &Command) -> Vec<Command> {
        let mut translated_commands = Vec::new();
        match command.data1 {
            0 => {
                let value = command.data2;
                // Value is red from 0 to 255
                //
                self.material_color[0] = value as f32 / 255.0;
                translated_commands.push(Command {
                    command_type: CommandType::SetMaterialRed,
                    data1: self.material_color[0].to_bits(),
                    data2: 0,
                });
            }
            1 => {
                let value = command.data2;
                // Value is green from 0 to 255
                //
                self.material_color[1] = value as f32 / 255.0;
                translated_commands.push(Command {
                    command_type: CommandType::SetMaterialGreen,
                    data1: self.material_color[1].to_bits(),
                    data2: 1,
                });
            }
            2 => {
                let value = command.data2;
                // Value is blue from 0 to 255
                //
                self.material_color[2] = value as f32 / 255.0;
                translated_commands.push(Command {
                    command_type: CommandType::SetMaterialBlue,
                    data1: self.material_color[2].to_bits(),
                    data2: 2,
                });
            }
            3 => {
                let value = command.data2;
                // Value is alpha from 0 to 255
                //
                self.material_color[3] = value as f32 / 255.0;
                translated_commands.push(Command {
                    command_type: CommandType::SetMaterialAlpha,
                    data1: self.material_color[3].to_bits(),
                    data2: 3,
                });
            }

            _ => {}
        }
        self.invalidate_selection_render_cache = true;
        translated_commands
    }

    /// Get the view from the camera.
    pub fn build_camera_projection(&self) -> Matrix4<f32> {
        Perspective3::new(
            1.0,
            std::f32::consts::PI / 4.0, // 45 degrees
            1.0,
            200.0,
        )
        .into_inner()
    }

    // Convert from 2d window coordinates to 3d world coordinates
    pub fn unproject(&self, x: f32, y: f32) -> Option<(Point3<f32>, Point3<f32>)> {
        // We need to calculate the model matrix for the drawable object
        let eye = self.camera.eye;
        let target = self.camera.target;
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        let model: Isometry3<f32> = Isometry3::identity();
        let projection_matrix = self.build_camera_projection();

        let model_view = (view * model).to_homogeneous();

        let transform_matrix = (projection_matrix * (model_view)).try_inverse().unwrap();

        let near = transform_matrix.transform_point(&Point3::new(x, y, 0.0));
        let far = transform_matrix.transform_point(&Point3::new(x, y, 1.0));

        Some((near, far))
    }

    pub fn handle_mouse_click(&mut self, command: &Command) {
        let current_position =
            Point2::new(f32::from_bits(command.data1), f32::from_bits(command.data2));
        if current_position.y > -0.6 {
            println!("Mouse clicked at position: {:?}", current_position);
            let maybe_near_far = self.unproject(current_position.x, current_position.y);
            if let Some((near, far)) = maybe_near_far {
                println!("Near: {:?}, Far: {:?}", near, far);

                self.model.paint_first_collision(
                    near,
                    far,
                    self.material_color,
                    self.noise as i32,
                    self.fluid as i32,
                );
                self.invalidate_drawables_cache = true;
                self.model.recalculate_occlusion();
                self.invalidate_render_cache = true;
            }
        }
    }

    pub fn handle_pick_material(&mut self, command: &Command) -> Vec<Command> {
        let mut translated_commands = Vec::new();
        translated_commands.push(Command {
            command_type: CommandType::CurrentMaterialRed,
            data1: self.material_color[0].to_bits(),
            data2: command.data2,
        });
        translated_commands.push(Command {
            command_type: CommandType::CurrentMaterialGreen,
            data1: self.material_color[1].to_bits(),
            data2: command.data2,
        });
        translated_commands.push(Command {
            command_type: CommandType::CurrentMaterialBlue,
            data1: self.material_color[2].to_bits(),
            data2: command.data2,
        });
        translated_commands.push(Command {
            command_type: CommandType::CurrentMaterialAlpha,
            data1: self.material_color[3].to_bits(),
            data2: command.data2,
        });
        translated_commands.push(Command {
            command_type: CommandType::CurrentMaterialNoise,
            data1: if self.noise { 1 } else { 0 },
            data2: command.data2,
        });
        translated_commands.push(Command {
            command_type: CommandType::CurrentMaterialFluid,
            data1: if self.fluid { 1 } else { 0 },
            data2: command.data2,
        });
        translated_commands
    }

    /// Handle the mouse scroll.
    pub fn handle_mouse_scroll(&mut self, command: &Command) {
        let direction: u32 = command.data2;
        let max_selection_radius: u32 = 128;
        let min_selection_radius: u32 = 1;
        if direction > 0 {
            self.selection_radius = min(self.selection_radius + 1, max_selection_radius);
        } else {
            self.selection_radius = max(self.selection_radius - 1, min_selection_radius);
        }
        self.invalidate_render_cache = true;
    }

    pub fn print_keyboard_bindings(&self) {
        println!("");
        println!("Keyboard Bindings:");
        println!("W or <up>: Move forward");
        println!("S or <down>: Move backward");
        println!("A or <left>: Move left");
        println!("D or <right>: Move right");
        println!("Q: Move up");
        println!("E: Move down");
        println!("I or 8: Move selection forward");
        println!("K or 5: Move selection backward");
        println!("J or 4: Move selection left");
        println!("L or 6: Move selection right");
        println!("U or 7: Move selection up");
        println!("O or 9: Move selection down");
        println!("Space: Create/Destroy voxels in the current selection");
        println!("T: Cycle the selection shape");
        println!("F: Toggle fluid mode");
        println!("G: Toggle grid visibility");
        println!("N: Toggle material noise");
    }

    pub fn more_red(&mut self) {
        self.material_color[0] += 0.1;
    }

    pub fn more_green(&mut self) {
        self.material_color[1] += 0.1;
    }

    pub fn more_blue(&mut self) {
        self.material_color[2] += 0.1;
    }

    pub fn more_alpha(&mut self) {
        self.material_color[3] += 0.1;
    }

    pub fn less_red(&mut self) {
        self.material_color[0] -= 0.1;
    }

    pub fn less_green(&mut self) {
        self.material_color[1] -= 0.1;
    }

    pub fn less_blue(&mut self) {
        self.material_color[2] -= 0.1;
    }

    pub fn less_alpha(&mut self) {
        self.material_color[3] -= 0.1;
    }

    /// Handle a key press.
    pub fn handle_key_down(&mut self, command: &Command) {
        let mut key = command.data1;

        println!("Key pressed: {}", key);
        if std::env::consts::OS == "macos" {
            key += 8;
        }
        match key {
            1 => self.select_file_to_open(),

            2 => self.select_file_to_save(),
            // Q
            16 => self.handle_move_up(),
            // E
            18 => self.handle_move_down(),
            // A or LEFT
            30 | 105 => self.handle_move_left(),
            // D or RIGHT
            32 | 106 => self.handle_move_right(),
            // W or UP
            17 | 103 => self.handle_move_forward(),
            // S or DOWN
            31 | 108 => self.handle_move_backward(),
            // SPACEBAR
            57 => self.handle_toggle_voxel(),
            // 4 or J
            36 | 75 => self.handle_move_selection_left(),
            // 6 or L
            38 | 77 => self.handle_move_selection_right(),
            // 2 or I
            23 | 72 => self.handle_move_selection_forward(),
            // 5 or K
            37 | 76 => self.handle_move_selection_backward(),
            // 7 | U
            22 | 71 => self.handle_move_selection_up(),
            // 9 | O
            24 | 73 => self.handle_move_selection_down(),
            // T
            20 => self.handle_toggle_selection_shape(),
            // F
            33 => self.toggle_fluid(),
            // G
            34 => self.toggle_show_grid(),
            // N
            49 => self.toggle_noise(),
            59 => self.more_red(),
            60 => self.more_green(),
            61 => self.more_blue(),
            62 => self.more_alpha(),
            63 => self.less_red(),
            64 => self.less_green(),
            65 => self.less_blue(),
            66 => self.less_alpha(),

            _ => log::info!("Unhandled key press: {}", key),
        }
    }

    pub fn update_current_material_red(&mut self, command: &Command) -> Vec<Command> {
        let mut translated_commands = Vec::<Command>::new();
        let red = f32::from_bits(command.data1);

        self.material_color[0] = red;
        self.selection_cube.color = [
            self.material_color[0],
            self.material_color[1],
            self.material_color[2],
            0.5,
        ];
        translated_commands.push(Command {
            command_type: CommandType::SetMaterialRed,
            data1: command.data1,
            data2: 0,
        });
        translated_commands
    }

    pub fn update_current_material_green(&mut self, command: &Command) -> Vec<Command> {
        let mut translated_commands = Vec::<Command>::new();
        let green = f32::from_bits(command.data1);

        self.material_color[1] = green;
        self.selection_cube.color = [
            self.material_color[0],
            self.material_color[1],
            self.material_color[2],
            0.5,
        ];
        translated_commands.push(Command {
            command_type: CommandType::SetMaterialGreen,
            data1: command.data1,
            data2: 1,
        });
        translated_commands
    }

    pub fn update_current_material_blue(&mut self, command: &Command) -> Vec<Command> {
        let mut translated_commands = Vec::<Command>::new();
        let blue = f32::from_bits(command.data1);

        self.material_color[2] = blue;
        self.selection_cube.color = [
            self.material_color[0],
            self.material_color[1],
            self.material_color[2],
            0.5,
        ];
        translated_commands.push(Command {
            command_type: CommandType::SetMaterialBlue,
            data1: command.data1,
            data2: 2,
        });
        translated_commands
    }

    pub fn update_current_material_alpha(&mut self, command: &Command) -> Vec<Command> {
        let mut translated_commands = Vec::<Command>::new();
        let alpha = f32::from_bits(command.data1);

        self.material_color[3] = alpha;
        self.selection_cube.color = [
            self.material_color[0],
            self.material_color[1],
            self.material_color[2],
            0.5,
        ];
        translated_commands.push(Command {
            command_type: CommandType::SetMaterialAlpha,
            data1: command.data1,
            data2: 3,
        });
        translated_commands
    }

    /// Process the command queue.
    pub fn process_commands(&mut self) -> Vec<Command> {
        let mut command_opt = self.command_input.next();
        let mut translated_commands = Vec::<Command>::new();

        while let Some(command) = command_opt {
            match command.command_type {
                CommandType::SliderMoved => {
                    translated_commands.extend(self.handle_slider_moved(&command));
                }
                CommandType::MouseDown => {
                    self.handle_mouse_down();
                }
                CommandType::MouseUp => {
                    self.handle_mouse_up();
                }
                CommandType::MouseClick => {
                    self.handle_mouse_click(&command);
                }
                CommandType::MouseMoved => {
                    self.handle_mouse_moved(&command);
                }
                CommandType::KeyDown => {
                    self.handle_key_down(&command);
                }
                CommandType::MouseScroll => {
                    self.handle_mouse_scroll(&command);
                }
                CommandType::PickMaterial => {
                    translated_commands.extend(self.handle_pick_material(&command));
                }
                CommandType::UpdateCurrentMaterialRed => {
                    translated_commands.extend(self.update_current_material_red(&command));
                }
                CommandType::UpdateCurrentMaterialGreen => {
                    translated_commands.extend(self.update_current_material_green(&command));
                }
                CommandType::UpdateCurrentMaterialBlue => {
                    translated_commands.extend(self.update_current_material_blue(&command));
                }
                CommandType::UpdateCurrentMaterialAlpha => {
                    translated_commands.extend(self.update_current_material_alpha(&command));
                }
                _ => {}
            }

            command_opt = self.command_input.next();
        }
        translated_commands
    }

    /// Should we render the current frame?
    pub fn throttle(&mut self) -> bool {
        if !self.dirty {
            return true;
        }

        if self.loading {
            return true;
        }

        if !self.drawing {
            return true;
        }

        self.throttle -= 1;
        if self.throttle >= 1 {
            return true;
        }
        self.throttle = 2;

        let now = Instant::now();
        let target_fps = self.target_fps;
        let target_delay = Duration::from_millis(1000 / target_fps as u64);
        if self.last_draw.is_some() {
            let last = self.last_draw.expect("last_draw is None");
            if now.duration_since(last).cmp(&target_delay).is_lt() {
                return true;
            }
        }
        self.last_draw = Some(now);
        false
    }

    /// Change the active color for this scene.

    /// Load a scene from the browser.

    /// Enable color noise.
    pub fn toggle_noise(&mut self) {
        self.noise = !self.noise;
        self.invalidate_render_cache = true;
    }

    /// Show grid.
    pub fn toggle_show_grid(&mut self) {
        self.grid_visible = !self.grid_visible;
    }

    /// Enable fluid.
    pub fn toggle_fluid(&mut self) {
        self.fluid = !self.fluid;
        self.invalidate_render_cache = true;
    }

    /// Load the default scene.

    /// Init the scene.
    pub fn init(&mut self) {
        self.render_cache = Some(HashMap::new());
        self.selection_vertices_cache = Some(Vec::new());
        self.light.eye = Point3::new(60.0, 60.0, 60.0);
        self.light.target = Point3::new(0.0, 0.0, 0.0);
        self.selection_cube.scale = 0.8f32;
        self.selection_cube.color = [0.8, 0.8, 0.8, 0.5];
        self.selection_cube.init();
        self.grid_xz.init();
        self.grid_xz.rotate([90.0_f32.to_radians(), 0.0, 0.0]);

        self.model.init();
        self.start_time = Some(Instant::now());

        self.print_keyboard_bindings();
        self.invalidate_render_cache = true;
    }

    /// Quicker than distance - no sqrt.
    pub fn calculate_distance_squared(from: &[i32; 3], to: &[i32; 3]) -> i32 {
        (from[0] - to[0]).pow(2) + (from[1] - to[1]).pow(2) + (from[2] - to[2]).pow(2)
    }

    /// Generate voxels based on selection.
    pub fn selection_voxels(
        center: &[i32; 3],
        radius: i32,
        shape: SelectionShape,
    ) -> Vec<[i32; 3]> {
        let mut voxels = Vec::new();
        let range: i32 = Ocnode::range() * 2;
        let radius_squared: i32 = radius.pow(2);
        let xmin = i32::max(center[0] - radius - 1, -range);
        let xmax = i32::min(center[0] + radius + 1, range);
        let ymin = i32::max(center[1] - radius - 1, -range);
        let ymax = i32::min(center[1] + radius + 1, range);
        let zmin = i32::max(center[2] - radius - 1, -range);
        let zmax = i32::min(center[2] + radius + 1, range);

        println!("Generating selection voxels...");
        if shape == SelectionShape::Sphere {
            for x in xmin..xmax {
                for y in ymin..ymax {
                    for z in zmin..zmax {
                        let voxel_position = [x, y, z];
                        let distance: i32 =
                            Self::calculate_distance_squared(center, &voxel_position);

                        if distance < radius_squared {
                            voxels.push([x, y, z]);
                        }
                    }
                }
            }
        } else if shape == SelectionShape::Cube {
            for x in xmin..xmax {
                for y in ymin..ymax {
                    for z in zmin..zmax {
                        let voxel_position = [x, y, z];
                        if (center[0] - voxel_position[0]).abs() < radius
                            && (center[1] - voxel_position[1]).abs() < radius
                            && (center[2] - voxel_position[2]).abs() < radius
                        {
                            voxels.push([x, y, z]);
                        }
                    }
                }
            }
        } else if shape == SelectionShape::SquareXZ {
            // SquareXZ
            for x in xmin..xmax {
                for z in zmin..zmax {
                    let voxel_position = [x, center[1], z];
                    if (center[0] - voxel_position[0]).abs() < radius
                        && (center[2] - voxel_position[2]).abs() < radius
                    {
                        voxels.push([x, center[1], z]);
                    }
                }
            }
        } else if shape == SelectionShape::SquareXY {
            // SquareXY
            for x in xmin..xmax {
                for y in ymin..ymax {
                    let voxel_position = [x, y, center[2]];
                    if (center[0] - voxel_position[0]).abs() < radius
                        && (center[1] - voxel_position[1]).abs() < radius
                    {
                        voxels.push([x, y, center[2]]);
                    }
                }
            }
        } else if shape == SelectionShape::SquareYZ {
            // SquareYZ
            for y in ymin..ymax {
                for z in zmin..zmax {
                    let voxel_position = [center[0], y, z];
                    if (center[1] - voxel_position[1]).abs() < radius
                        && (center[2] - voxel_position[2]).abs() < radius
                    {
                        voxels.push([center[0], y, z]);
                    }
                }
            }
        } else if shape == SelectionShape::CircleXZ {
            // CircleXZ
            for x in xmin..xmax {
                for z in zmin..zmax {
                    let voxel_position = [x, center[1], z];
                    if (((center[0] - voxel_position[0]).abs() as f64).powi(2)
                        + ((center[2] - voxel_position[2]).abs() as f64).powi(2))
                    .sqrt()
                        < radius as f64
                    {
                        voxels.push([x, center[1], z]);
                    }
                }
            }
        } else if shape == SelectionShape::CircleXY {
            // CircleXY
            for x in xmin..xmax {
                for y in ymin..ymax {
                    let voxel_position = [x, y, center[2]];
                    if (((center[0] - voxel_position[0]).abs() as f64).powi(2)
                        + ((center[1] - voxel_position[1]).abs() as f64).powi(2))
                    .sqrt()
                        < radius as f64
                    {
                        voxels.push([x, y, center[2]]);
                    }
                }
            }
        } else if shape == SelectionShape::CircleYZ {
            // CircleYZ
            for y in ymin..ymax {
                for z in zmin..zmax {
                    let voxel_position = [center[0], y, z];
                    if (((center[1] - voxel_position[1]).abs() as f64).powi(2)
                        + ((center[2] - voxel_position[2]).abs() as f64).powi(2))
                    .sqrt()
                        < radius as f64
                    {
                        voxels.push([center[0], y, z]);
                    }
                }
            }
        }

        voxels
    }

    /// Draw the scene.
    pub fn draw(
        &mut self,
        display: &Display<WindowSurface>,
        frame: &mut Frame,
        graphics: &mut Graphics,
    ) {
        let animation_speed = 0.1;
        self.elapsed = self
            .start_time
            .expect("Scene was not initialised")
            .elapsed()
            .as_secs_f32()
            * animation_speed;

        graphics.prepare_camera_frame(frame);

        if self.invalidate_render_cache
            || self.invalidate_selection_render_cache
            || self.invalidate_render_material.is_some()
        {
            if self.invalidate_render_cache {
                self.render_cache
                    .as_mut()
                    .expect("Render cache should be initialized")
                    .clear();
                self.invalidate_render_cache = false;
                self.invalidate_selection_render_cache = true;
            }

            /*graphics.prepare_shadow_frame();

            for voxel in self.model.drawables().iter() {
                graphics.draw_shadow(display, voxel, self.light);
            }

            graphics.finish_shadow_frame();
            */

            if self.invalidate_selection_render_cache {
                self.invalidate_selection_render_cache = false;
                self.selection_vertices_cache.as_mut().unwrap().clear();

                for selection in &Self::selection_voxels(
                    &self.selection_position,
                    self.selection_radius as i32,
                    self.selection_shape,
                ) {
                    self.selection_cube.translation = [
                        selection[0] as f32 + 0.1,
                        selection[1] as f32 + 0.1,
                        selection[2] as f32 + 0.1,
                    ];

                    let vertices = self.selection_cube.vertices_world();

                    println!("Rebuilding selection render cache X number of selection cubes.");
                    self.selection_vertices_cache
                        .as_mut()
                        .unwrap()
                        .extend(vertices);
                }
            }

            if self.invalidate_drawables_cache {
                self.invalidate_drawables_cache = false;
                let mut drawables: Vec<Cube> = self.model.drawables();

                let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
                drawables.sort_by(|a, b| {
                    let a_dist = a.depth(camera_eye);
                    let b_dist = b.depth(camera_eye);

                    b_dist.partial_cmp(&a_dist).unwrap()
                });
                self.drawables_cache = drawables;
                self.drawables_cache = self.model.drawables();
            }

            for voxel in self.drawables_cache.iter() {
                let vertices = voxel.vertices_world();
                let material = Material::new(voxel.color, voxel.noise, voxel.fluid);
                if self.invalidate_render_material.is_none()
                    || self.invalidate_render_material.as_ref().unwrap() == &material
                {
                    println!("Rebuilding material render cache X number of cubes");
                    self.render_cache
                        .as_mut()
                        .expect("Render cache should be initialized")
                        .entry(material)
                        .or_insert_with(Vec::new)
                        .extend(vertices);
                }
            }

            self.invalidate_render_material = None;
        }
        for material in self
            .render_cache
            .as_ref()
            .expect("Render cache should be initialized")
            .keys()
        {
            // Process each material here
            graphics.draw_vertices(
                display,
                frame,
                material,
                self.render_cache
                    .as_ref()
                    .expect("Render cache should be initialized")
                    .get(material)
                    .unwrap(),
                self.camera,
                self.light,
                self.elapsed,
            );
        }
        let material = Material::new(self.material_color, self.noise as i32, self.fluid as i32);

        graphics.draw_vertices(
            display,
            frame,
            &material,
            self.selection_vertices_cache.as_mut().expect("Some"),
            self.camera,
            self.light,
            self.elapsed,
        );

        if self.grid_visible {
            graphics.draw(
                display,
                frame,
                &self.grid_xz,
                self.camera,
                self.light,
                self.elapsed,
            );
        }

        graphics.finish_camera_frame();

        // We are only rendering when idle, so we can skip the throttling.
        // Continuous rendering is needed to animate the fluid.
        //scene.dirty = false;
    }
}
