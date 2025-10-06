use crate::cube::Cube;
use crate::octree::Octree;
use crate::storage::Storage;

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

    /// Call optimize on the nested OcNodes
    pub fn optimize(&mut self, camera_eye: [f32; 3]) {
        self.voxels.optimize(camera_eye);
    }

    /// Initialise
    pub fn init(&mut self) {
        self.voxels.init();
    }

    /// Set the name of the scene. Used to save/restore.
    pub fn set_name(&mut self, name: String) {
        self.voxels.set_name(name);
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

    /// Delete a scene from browser indexeddb
    pub async fn delete_scene(&self) {
        let storage = Storage::new();
        if self.voxels.name != "Default" {
            _ = storage.delete_scene(self.voxels.name.to_string()).await;
        }
    }

    /// Save a scene to browser indexeddb
    pub async fn save(&self) {
        let storage = Storage::new();

        let serial = self.voxels.prepare();
        _ = storage.save(serial).await;
    }
}
