use crate::command::{Command, CommandType};
use crate::command_queue::CommandQueue;
use crate::drawable::Drawable;
use crate::graphics::Graphics;
use crate::grid::Grid;
use crate::model::Model;
use crate::mouse::Mouse;
use crate::ocnode::Ocnode;
use crate::storage::Storage;
use crate::stored_octree::StoredOctree;
use crate::{camera::Camera, cube::Cube};
use glium::Frame;
use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;
use nalgebra::{Point2, Point3, Vector3};
use std::cmp::{max, min};
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
    fluid: i32,
    /// Is the material noisy?
    noise: i32,
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
    /// Only recalculate the selection voxels cache if the scene has changed.
    selection_cache: Vec<[i32; 3]>,
    /// Flag to asynchronously invalidate the vertices cache in the graphics pipeline.
    invalidate_vertices: bool,
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
            fluid: 1,
            noise: 0,
            dirty: true,
            elapsed: 0.0,
            last_draw: None,
            grid_visible: true,
            target_fps: 30,
            drawables_cache: Vec::new(),
            selection_cache: Vec::new(),
            invalidate_vertices: false,
        }
    }

    /// Helper function to rotate a point around an axis.
    fn rotate_2d(target: Point2<f32>, pivot: Point2<f32>, angle_radians: f32) -> Point2<f32> {
        // Precalculate the cosine
        let angle_sin = f32::sin(angle_radians);
        let angle_cos = f32::cos(angle_radians);

        // Subtract the pivot from the target
        let focused = target - pivot;
        // Rotate
        let rotated = Point2::new(
            focused.x * angle_cos - focused.y * angle_sin,
            focused.x * angle_sin + focused.y * angle_cos,
        );

        // Add the pivot back
        Point2::new(rotated.x + pivot.x, rotated.y + pivot.y)
    }

    /// Add a command to the queue of commands to process later.
    pub fn queue_command(&mut self, command: Command) {
        self.dirty = true;

        self.command_input.queue_command(command);
    }

    /// Change this scene name.
    pub fn set_name(&mut self, name: String) {
        self.model.set_name(name);
    }

    /// Something changed - we need to recalculate the drawables cache.
    pub fn invalidate_drawables_cache(&mut self) {
        self.drawables_cache.clear();
    }

    /// Something changed - we need to recalculate the selection cache.
    pub fn invalidate_selection_cache(&mut self) {
        self.selection_cache.clear();
    }

    pub fn invalidate_vertices_cache(&mut self) {
        self.invalidate_vertices = true;
        self.model.recalculate_occlusion();
    }

    /// Process a mouse down event.
    pub fn handle_mouse_down(&mut self) {
        println!("Mouse was pressed");

        self.mouse.is_pressed = true;
    }

    /// Process a mouse up event.
    pub fn handle_mouse_up(&mut self) {
        println!("Mouse was lifted");

        self.mouse.is_pressed = false;
    }

    /// Process a mouse moved event.
    pub fn handle_mouse_moved(&mut self, command: &Command) {
        let current_position = Point2::new(command.data1 as i32, command.data2 as i32);

        if self.mouse.is_pressed {
            println!("Mouse was moved when pressed");

            let position_diff = Point2::new(
                current_position.x - self.mouse.last_position.x,
                current_position.y - self.mouse.last_position.y,
            );
            let current_camera_eye = self.camera.eye;
            let current_camera_target = self.camera.target;
            let current_camera_direction = current_camera_target - current_camera_eye;

            let current_camera_distance = (current_camera_direction.x.powf(2.0f32)
                + current_camera_direction.z.powf(2.0f32))
            .sqrt();
            let scale = 20.0f32 / current_camera_distance;
            let scaled_direction = scale * current_camera_direction;
            let scaled_point =
                Point3::new(scaled_direction.x, scaled_direction.y, scaled_direction.z);
            let blunting = 100.0;
            let current_camera_eye_2d = Point2::new(current_camera_eye.x, current_camera_eye.z);
            let current_camera_target_2d = Point2::new(
                scaled_point.x + current_camera_eye.x,
                scaled_point.z + current_camera_eye.z,
            );
            // rotate the eye around the target
            let adjusted = Self::rotate_2d(
                current_camera_eye_2d,
                current_camera_target_2d,
                position_diff.x as f32 / blunting,
            );

            self.camera.eye = Point3::new(adjusted.x, current_camera_eye.y, adjusted.y);

            // Up down does not need rotation.

            self.camera.eye.y += position_diff.y as f32 / 10.0f32;

            let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
            self.model.optimize(camera_eye);
            self.invalidate_drawables_cache();
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
        self.invalidate_drawables_cache();
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
        self.invalidate_drawables_cache();
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
        self.invalidate_drawables_cache();
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
        self.invalidate_drawables_cache();
    }

    /// The key was pressed to move forward.
    pub fn handle_move_forward(&mut self) {
        let diff = self.camera.target - self.camera.eye;
        let blunting = 10.0;
        let projection = Vector3::new(diff.x, 0.0, diff.z) / blunting;

        self.camera.eye += projection;
        self.camera.target += projection;
        let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
        self.model.optimize(camera_eye);
        self.invalidate_drawables_cache();
    }

    /// The key was pressed to move backwards.
    pub fn handle_move_backward(&mut self) {
        let diff = self.camera.target - self.camera.eye;
        let blunting = 10.0;
        let projection = Vector3::new(-diff.x, 0.0, -diff.z) / blunting;

        self.camera.eye += projection;
        self.camera.target += projection;
        let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
        self.model.optimize(camera_eye);
        self.invalidate_drawables_cache();
    }

    /// The key was pressed to toggle the state of the current selection.
    pub fn handle_toggle_voxel(&mut self) {
        let selections = Self::selection_voxels(
            &self.selection_position,
            self.selection_radius as i32,
            self.selection_shape,
        );

        let value: bool = self.model.all_voxels_active(&selections);
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
        self.model.toggle_voxels(
            selections, !value, color, camera_eye, self.fluid, self.noise,
        );
        self.invalidate_selection_cache();
        self.invalidate_drawables_cache();
        self.invalidate_vertices_cache();
    }

    /// Save the scene to the browser.
    pub async fn save_scene(&self) {
        self.model.save().await;
    }

    /// Move the selection shape left.
    pub fn handle_move_selection_left(&mut self) {
        self.selection_cube.translate([-1.0, 0.0, 0.0]);
        self.selection_position[0] -= 1;
        self.invalidate_selection_cache();
    }

    /// Move the selection shape right.
    pub fn handle_move_selection_right(&mut self) {
        self.selection_cube.translate([1.0, 0.0, 0.0]);
        self.selection_position[0] += 1;
        self.invalidate_selection_cache();
    }

    /// Move the selection shape forward.
    pub fn handle_move_selection_forward(&mut self) {
        self.selection_cube.translate([0.0, 0.0, 1.0]);
        self.selection_position[2] += 1;
        self.invalidate_selection_cache();
    }

    /// Move the selection shape backward.
    pub fn handle_move_selection_backward(&mut self) {
        self.selection_cube.translate([0.0, 0.0, -1.0]);
        self.selection_position[2] -= 1;
        self.invalidate_selection_cache();
    }

    /// Move the selection shape up.
    pub fn handle_move_selection_up(&mut self) {
        self.selection_cube.translate([0.0, 1.0, 0.0]);
        self.selection_position[1] += 1;
        self.invalidate_selection_cache();
    }

    /// Move the selection shape down.
    pub fn handle_move_selection_down(&mut self) {
        self.selection_cube.translate([0.0, -1.0, 0.0]);
        self.selection_position[1] -= 1;
        self.invalidate_selection_cache();
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
        self.invalidate_selection_cache();
    }

    /// Handle the mouse scroll.
    pub fn handle_mouse_scroll(&mut self, command: &Command) {
        let direction: u32 = command.data2;
        let max_selection_radius: u32 = 32;
        let min_selection_radius: u32 = 1;
        println!("Handle mouse scroll {}", direction);
        if direction > 0 {
            self.selection_radius = min(self.selection_radius + 1, max_selection_radius);
        } else {
            self.selection_radius = max(self.selection_radius - 1, min_selection_radius);
        }
        println!("New selection radius {}", self.selection_radius);
        self.invalidate_selection_cache();
    }

    /// Handle a key press.
    pub fn handle_key_down(&mut self, command: &Command) {
        let key = command.data1;
        println!("Handle key down: {}", key);

        match key {
            // E
            18 => self.handle_move_up(),
            // C
            46 => self.handle_move_down(),
            // A or LEFT
            30 | 105 => self.handle_move_left(),
            // D or RIGHT
            32 | 106 => self.handle_move_right(),
            // W or UP
            17 | 103 => self.handle_move_forward(),
            // S or X or DOWN
            31 | 45 | 108 => self.handle_move_backward(),
            // SPACEBAR
            57 => self.handle_toggle_voxel(),
            // 4 or J
            36 | 75 => self.handle_move_selection_left(),
            // 6 or L
            38 | 77 => self.handle_move_selection_right(),
            // 2 or I
            23 | 80 => self.handle_move_selection_forward(),
            // 8 or K
            37 | 72 => self.handle_move_selection_backward(),
            // 9 | O
            24 | 73 => self.handle_move_selection_up(),
            // 3 | P
            25 | 81 => self.handle_move_selection_down(),
            // T
            //84 => self.handle_toggle_selection_shape(scene),
            _ => log::info!("Unhandled key press: {}", key),
        }
    }

    /// Process the command queue.
    pub fn process_commands(&mut self) {
        let mut command_opt = self.command_input.next();

        while let Some(command) = command_opt {
            match command.command_type {
                CommandType::MouseDown => {
                    self.handle_mouse_down();
                }
                CommandType::MouseUp => {
                    self.handle_mouse_up();
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
            }

            command_opt = self.command_input.next();
        }
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
    pub fn set_material_color(
        &mut self,
        red_str: &str,
        green_str: &str,
        blue_str: &str,
        alpha_str: &str,
    ) {
        log::debug!("Set material color ({red_str}, {green_str}, {blue_str}, {alpha_str})");
        let red = red_str.parse::<i32>().unwrap();
        let red_f32 = red as f32 / 255.0;
        let green = green_str.parse::<i32>().unwrap();
        let green_f32 = green as f32 / 255.0;
        let blue = blue_str.parse::<i32>().unwrap();
        let blue_f32 = blue as f32 / 255.0;
        let alpha_f32 = alpha_str.parse::<f32>().unwrap();

        self.material_color = [red_f32, green_f32, blue_f32, alpha_f32];
        self.selection_cube.color = [red_f32, green_f32, blue_f32, 0.5];
    }

    /// Load a scene from the browser.
    pub async fn load_scene(&mut self) {
        let name = {
            self.drawing = false;
            self.loading = true;
            self.model.voxels.name.clone()
        };

        let storage = Storage::new();
        let serial: Option<StoredOctree> = storage.load_scene(name).await;
        if serial.is_some() {
            let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
            self.model
                .voxels
                .load_from_serial(serial.unwrap(), camera_eye);
            self.drawing = true;
            self.loading = false;
        }
        self.invalidate_drawables_cache();
    }

    /// Delete a scene from the browser.
    pub async fn delete_scene(&mut self) {
        let model = {
            self.model.voxels.clear();
            self.model.clone()
        };
        model.delete_scene().await;
    }

    /// Enable color noise.
    pub async fn toggle_noise(&mut self) {
        self.noise = 1;
    }

    /// Enable smoothing.
    pub async fn toggle_smooth(&mut self) {
        self.noise = 0;
    }

    /// Enable solid material.
    pub async fn toggle_solid(&mut self) {
        log::error!("Fluid goes off");
        self.fluid = 0;
    }

    /// Show grid.
    pub async fn toggle_show_grid(&mut self) {
        log::error!("Grid goes on");
        self.grid_visible = true;
    }

    /// Show grid.
    pub async fn toggle_hide_grid(&mut self) {
        log::error!("Grid goes off");
        self.grid_visible = false;
    }

    /// Enable fluid.
    pub async fn toggle_fluid(&mut self) {
        log::error!("Fluid goes on");
        self.fluid = 1;
    }

    pub async fn set_target_fps(&mut self, fps: u32) {
        self.target_fps = fps;
    }

    /// Load the default scene.
    pub async fn load_first_scene(&mut self) {
        let storage = Storage::new();
        let serial: Option<StoredOctree> = storage.load_first_scene().await;
        if serial.is_some() {
            let camera_eye = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z];
            self.model
                .voxels
                .load_from_serial(serial.unwrap(), camera_eye);
            self.drawing = true;
            self.loading = false;
        } else {
            self.drawing = true;
            self.loading = false;
        }
    }

    /// Init the scene.
    pub fn init(&mut self) {
        self.light.eye = Point3::new(15.0, 60.0, 14.0);
        self.light.target = Point3::new(0.0, 0.0, 0.0);
        self.selection_cube.scale = 0.8f32;
        self.selection_cube.color = [0.8, 0.8, 0.8, 0.5];
        self.selection_cube.init();
        self.grid_xz.init();
        self.grid_xz.rotate([90.0_f32.to_radians(), 0.0, 0.0]);

        self.model.init();
        self.handle_toggle_voxel();
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

        if shape == SelectionShape::Sphere {
            for x in -range..range {
                for y in -range..range {
                    for z in -range..range {
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
            for x in -range..range {
                for y in -range..range {
                    for z in -range..range {
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
            for x in -range..range {
                for z in -range..range {
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
            for x in -range..range {
                for y in -range..range {
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
            for y in -range..range {
                for z in -range..range {
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
            for x in -range..range {
                for z in -range..range {
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
            for x in -range..range {
                for y in -range..range {
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
            for y in -range..range {
                for z in -range..range {
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
        self.elapsed = Instant::now().elapsed().as_secs_f32();

        if self.invalidate_vertices {
            self.invalidate_vertices = false;
            graphics.invalidate_vertices_cache();
        }
        graphics.prepare_shadow_frame();

        for voxel in self.model.drawables().iter() {
            graphics.draw_shadow(display, voxel, self.light);
        }

        graphics.finish_shadow_frame();

        graphics.prepare_camera_frame(frame);
        if self.selection_cache.len() == 0 {
            self.selection_cache = Self::selection_voxels(
                &self.selection_position,
                self.selection_radius as i32,
                self.selection_shape,
            );
        }
        for selection in &self.selection_cache {
            self.selection_cube.translation = [
                selection[0] as f32 + 0.1,
                selection[1] as f32 + 0.1,
                selection[2] as f32 + 0.1,
            ];
            graphics.draw(
                display,
                frame,
                &self.selection_cube,
                self.camera,
                self.light,
                self.elapsed,
            );
        }
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

        if self.drawables_cache.len() == 0 {
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
            graphics.draw(display, frame, voxel, self.camera, self.light, self.elapsed);
        }

        graphics.finish_camera_frame();

        // We are only rendering when idle, so we can skip the throttling.
        // Continuous rendering is needed to animate the fluid.
        //scene.dirty = false;
    }
}
