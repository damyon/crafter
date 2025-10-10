use crate::cube::Cube;
use crate::ocnode::Ocnode;
use crate::stored_octree::StoredOctree;

/// An octree has a name and a tree of nodes.
#[derive(Clone)]
pub struct Octree {
    pub name: String,
    root: Ocnode,
    depth: u32,
}

impl Octree {
    /// Create a new Octree
    pub const fn new() -> Octree {
        Octree {
            name: String::new(),
            root: Ocnode::new(),
            depth: 1,
        }
    }

    /// Get the full list of active nodes from the tree.
    pub fn active_nodes(&self) -> Vec<Ocnode> {
        self.root.active_nodes()
    }

    /// Hide all nodes in the tree.
    pub fn clear(&mut self) {
        self.root.clear();
    }

    pub fn recalculate_occlusion(&mut self) {
        let borrow = self.root.clone();
        println!("Recalculate occlusion");
        self.root.recalculate_occlusion(&borrow);
    }

    /// Optimize walks the tree and based on the camera position
    /// hides nested smaller cubes in bigger ones if the detail is not required.
    pub fn optimize(&mut self, camera_eye: [f32; 3]) {
        self.root.optimize(camera_eye);
    }

    /// Subdivide the tree into small cubes.
    pub fn init(&mut self) {
        // The LEVELS here is important. It defines the number of sub-divisions
        // so it exponentially increases the number of nodes.
        self.decimate(crate::ocnode::LEVELS);
    }

    /// Change the name of the scene.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Load the scene from browser indexeddb.
    pub fn load_from_serial(&mut self, source: StoredOctree, camera_eye: [f32; 3]) {
        self.name = source.name;

        self.root.clear();

        for node in source.active_nodes {
            self.root.apply(&node);
        }
        self.root.optimize(camera_eye);
    }

    /// Generate the list of drawables from the tree of cubes.
    pub fn drawables(&mut self) -> Vec<Cube> {
        self.root.drawables()
    }

    /// Subdivide the tree into smaller cubes.
    pub fn decimate(&mut self, sub_division_level: u32) {
        self.depth = sub_division_level;
        self.root.decimate(sub_division_level);
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
        self.root
            .toggle_voxels(&positions, value, color, fluid, noise);
        self.root.optimize(camera_eye);
    }

    /// Serialize the tree.
    pub fn prepare(&self) -> StoredOctree {
        StoredOctree {
            name: String::from(self.name.as_str()),
            active_nodes: self.active_nodes(),
        }
    }

    /// Check all indexes and determine if all nodes are active.
    pub fn all_voxels_active(&self, positions: &Vec<[i32; 3]>) -> bool {
        self.root.all_voxels_active(positions)
    }
}
