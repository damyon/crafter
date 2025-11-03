use crate::cube::Cube;
use crate::octree::Octree;
use crate::storage::Storage;
use nalgebra::Point3;

/// A model contains an Octree of voxels.
#[derive(Clone)]
pub struct Model {
    pub voxels: Octree,
}

impl Model {
    /// Create new empty model.
    pub const fn new() -> Model {
        Model {
            voxels: Octree::new(),
        }
    }

    /// Get the list of drawables from the OcTree
    pub fn drawables(&mut self) -> Vec<Cube> {
        self.voxels.drawables()
    }

    pub fn paint_first_collision(
        &mut self,
        near: Point3<f32>,
        far: Point3<f32>,
        material_color: [f32; 4],
        noise: i32,
        fluid: i32,
    ) {
        self.voxels
            .paint_first_collision(near, far, material_color, noise, fluid);
    }

    /// Call optimize on the nested OcNodes
    pub fn optimize(&mut self, camera_eye: [f32; 3]) {
        self.voxels.optimize(camera_eye);
    }

    pub fn recalculate_occlusion(&mut self) {
        self.voxels.recalculate_occlusion();
    }

    /// Initialise
    pub fn init(&mut self) {
        self.voxels.init();
    }

    pub fn toggle_voxels(
        &mut self,
        positions: Vec<[i32; 3]>,
        value: bool,
        color: [f32; 4],
        camera_eye: [f32; 3],
        fluid: i32,
        noise: i32,
    ) {
        self.voxels
            .toggle_voxels(positions, value, color, camera_eye, fluid, noise);
    }

    /// Determine if all voxels in the list are active.
    pub fn all_voxels_active(&self, positions: &Vec<[i32; 3]>) -> bool {
        self.voxels.all_voxels_active(positions)
    }

    /// Save a scene to browser indexeddb
    pub fn save(&self, path: &str) {
        let storage = Storage::new(path);

        let serial = self.voxels.prepare();
        _ = storage.save(serial);
    }

    /// Save a scene to browser indexeddb
    pub fn load(&mut self, path: &str, camera_eye: [f32; 3]) {
        let storage = Storage::new(path);

        let loaded = storage.load_first_scene().unwrap();
        self.voxels.load_from_serial(loaded, camera_eye);
    }
}
