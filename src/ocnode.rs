use crate::{cube::Cube, drawable::Drawable};
use nalgebra::Point3;
use serde::{Deserialize, Serialize};

/// Helper function to create an empty list.
/// The scope is odd.
fn empty_list() -> [Option<Box<Ocnode>>; 8] {
    [None, None, None, None, None, None, None, None]
}

pub const LEVELS: u32 = 8;

/// A struct representing a single cube for the octree.
/// Cubes contain children which are smaller cubes.
#[derive(Serialize, Deserialize, Clone)]
pub struct Ocnode {
    /// the x index of the cube.
    #[serde(rename = "x")]
    x_index: i32,
    /// the y index of the cube.
    #[serde(rename = "y")]
    y_index: i32,
    /// the z index of the cube.
    #[serde(rename = "z")]
    z_index: i32,
    /// How many parents does this cube have.
    #[serde(rename = "level")]
    sub_division_level: u32,
    /// Is this cube empty or filled?
    active: bool,
    /// We don't serialize this directly but this is the smaller cubes inside this one.
    #[serde(skip)]
    #[serde(default = "empty_list")]
    children: [Option<Box<Self>>; 8],
    /// Does this cube contain smaller ones?
    has_children: bool,
    /// The color of the cube including alpha channel.
    color: [f32; 4],
    /// Render this node with fluid animation.
    fluid: i32,
    /// Render this node with a noisy texture.
    noise: i32,
    front_occluded_calculated: bool,
    back_occluded_calculated: bool,
    top_occluded_calculated: bool,
    bottom_occluded_calculated: bool,
    left_occluded_calculated: bool,
    right_occluded_calculated: bool,
}

impl Ocnode {
    /// Create a new empty cube.
    pub const fn new() -> Ocnode {
        Ocnode {
            x_index: -Ocnode::range(),
            y_index: -Ocnode::range(),
            z_index: -Ocnode::range(),
            sub_division_level: 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: [0.8, 0.8, 0.8, 0.8],
            fluid: 0,
            noise: 0,
            front_occluded_calculated: false,
            back_occluded_calculated: false,
            top_occluded_calculated: false,
            bottom_occluded_calculated: false,
            left_occluded_calculated: false,
            right_occluded_calculated: false,
        }
    }

    pub fn intersects_line(&self, near: Point3<f32>, far: Point3<f32>) -> bool {
        // 6 planes form the cube.
        // foreach plane - find the intersection point
        // if intersection point is within the cube, return true
        //
        let min_vertex: Point3<f32> = Point3::new(
            self.x_index as f32 * self.resolution(self.sub_division_level) as f32,
            self.y_index as f32 * self.resolution(self.sub_division_level) as f32,
            self.z_index as f32 * self.resolution(self.sub_division_level) as f32,
        );
        let max_vertex = Point3::new(
            (self.x_index + 1) as f32 * self.resolution(self.sub_division_level) as f32,
            (self.y_index + 1) as f32 * self.resolution(self.sub_division_level) as f32,
            (self.z_index + 1) as f32 * self.resolution(self.sub_division_level) as f32,
        );

        // line equation = near + t * (far - near)

        // front
        let front_plane_z = self.z_index as f32;
        // front_plane_z = near.z + t * (far.z - near.z)
        // front_plane_z - near.z = t * (far.z - near.z)
        // (front_plane_z - near.z) / (far.z - near.z)=t
        let t = (front_plane_z - near.z) / (far.z - near.z);
        let intersection = near + t * (far - near);
        if (intersection.x >= min_vertex.x && intersection.x <= max_vertex.x)
            && (intersection.y >= min_vertex.y && intersection.y <= max_vertex.y)
            && (intersection.z >= min_vertex.z && intersection.z <= max_vertex.z)
        {
            return true;
        }
        // left
        let left_plane_x = self.x_index as f32;
        let t = (left_plane_x - near.x) / (far.x - near.x);
        let intersection = near + t * (far - near);
        if (intersection.x >= min_vertex.x && intersection.x <= max_vertex.x)
            && (intersection.y >= min_vertex.y && intersection.y <= max_vertex.y)
            && (intersection.z >= min_vertex.z && intersection.z <= max_vertex.z)
        {
            return true;
        }
        // right
        let right_plane_x = (self.x_index + 1) as f32;
        let t = (right_plane_x - near.x) / (far.x - near.x);
        let intersection = near + t * (far - near);
        if (intersection.x >= min_vertex.x && intersection.x <= max_vertex.x)
            && (intersection.y >= min_vertex.y && intersection.y <= max_vertex.y)
            && (intersection.z >= min_vertex.z && intersection.z <= max_vertex.z)
        {
            return true;
        }
        // back
        let back_plane_y = self.y_index as f32;
        let t = (back_plane_y - near.y) / (far.y - near.y);
        let intersection = near + t * (far - near);
        if (intersection.x >= min_vertex.x && intersection.x <= max_vertex.x)
            && (intersection.y >= min_vertex.y && intersection.y <= max_vertex.y)
            && (intersection.z >= min_vertex.z && intersection.z <= max_vertex.z)
        {
            return true;
        }
        // top
        let top_plane_y = (self.y_index + 1) as f32;
        let t = (top_plane_y - near.y) / (far.y - near.y);
        let intersection = near + t * (far - near);
        if (intersection.x >= min_vertex.x && intersection.x <= max_vertex.x)
            && (intersection.y >= min_vertex.y && intersection.y <= max_vertex.y)
            && (intersection.z >= min_vertex.z && intersection.z <= max_vertex.z)
        {
            return true;
        }
        // bottom
        let bottom_plane_y = self.y_index as f32;
        let t = (bottom_plane_y - near.y) / (far.y - near.y);
        let intersection = near + t * (far - near);
        if (intersection.x >= min_vertex.x && intersection.x <= max_vertex.x)
            && (intersection.y >= min_vertex.y && intersection.y <= max_vertex.y)
            && (intersection.z >= min_vertex.z && intersection.z <= max_vertex.z)
        {
            return true;
        }

        false
    }

    pub fn distance_to(&self, point: Point3<f32>) -> f32 {
        let dx = point.x - self.x_index as f32;
        let dy = point.y - self.y_index as f32;
        let dz = point.z - self.z_index as f32;
        dx * dx + dy * dy + dz * dz
    }

    pub fn paint_connected_nodes(
        &mut self,
        collision: (i32, i32, i32, u32),
        material_color: [f32; 4],
        noise: i32,
        fluid: i32,
    ) {
        let mut completed = Vec::new();
        self.paint_connected_nodes_with_completion(
            collision,
            material_color,
            noise,
            fluid,
            completed.as_mut(),
        );
    }

    pub fn paint_connected_nodes_with_completion(
        &mut self,
        collision: (i32, i32, i32, u32),
        material_color: [f32; 4],
        noise: i32,
        fluid: i32,
        completed: &mut Vec<(i32, i32, i32, u32)>,
    ) {
        let (x, y, z, level) = collision;
        let candidate_opt = self.find_mut_by_index(x, y, z, level);
        let left_occluded: bool;
        let right_occluded: bool;
        let top_occluded: bool;
        let bottom_occluded: bool;
        let front_occluded: bool;
        let back_occluded: bool;

        println!("Completed length: {}", completed.len());
        if let Some(candidate) = candidate_opt {
            println!("Push completion vector");
            completed.push((x, y, z, level));
            candidate.color = material_color;
            candidate.noise = noise;
            candidate.fluid = fluid;
            left_occluded = candidate.left_occluded_calculated;
            right_occluded = candidate.right_occluded_calculated;
            top_occluded = candidate.top_occluded_calculated;
            bottom_occluded = candidate.bottom_occluded_calculated;
            front_occluded = candidate.front_occluded_calculated;
            back_occluded = candidate.back_occluded_calculated;
        } else {
            println!("Could not find candidate");
            return;
        }

        if left_occluded {
            if !completed.contains(&(x - 1, y, z, level)) {
                self.paint_connected_nodes_with_completion(
                    (x - 1, y, z, level),
                    material_color,
                    noise,
                    fluid,
                    completed,
                );
            }
        }
        if right_occluded {
            if !completed.contains(&(x + 1, y, z, level)) {
                self.paint_connected_nodes_with_completion(
                    (x + 1, y, z, level),
                    material_color,
                    noise,
                    fluid,
                    completed,
                );
            }
        }
        if top_occluded {
            if !completed.contains(&(x, y + 1, z, level)) {
                self.paint_connected_nodes_with_completion(
                    (x, y + 1, z, level),
                    material_color,
                    noise,
                    fluid,
                    completed,
                );
            }
        }
        if bottom_occluded {
            if !completed.contains(&(x, y - 1, z, level)) {
                self.paint_connected_nodes_with_completion(
                    (x, y - 1, z, level),
                    material_color,
                    noise,
                    fluid,
                    completed,
                );
            }
        }
        if front_occluded {
            if !completed.contains(&(x, y, z - 1, level)) {
                println!("Move forward");
                self.paint_connected_nodes_with_completion(
                    (x, y, z - 1, level),
                    material_color,
                    noise,
                    fluid,
                    completed,
                );
            }
        }
        if back_occluded {
            if !completed.contains(&(x, y, z + 1, level)) {
                println!("Move backwards");
                self.paint_connected_nodes_with_completion(
                    (x, y, z + 1, level),
                    material_color,
                    noise,
                    fluid,
                    completed,
                );
            }
        }
    }

    pub fn find_first_collision(
        &self,
        near: Point3<f32>,
        far: Point3<f32>,
    ) -> Option<(i32, i32, i32, u32)> {
        let active = self.active_nodes();
        let mut hits: Vec<&Ocnode> = active
            .iter()
            .filter(|node| node.intersects_line(near, far))
            .collect();

        hits.sort_unstable_by(|a, b| {
            a.distance_to(near)
                .partial_cmp(&b.distance_to(near))
                .unwrap()
        });
        if hits.len() > 0 {
            Some((
                hits[0].x_index,
                hits[0].y_index,
                hits[0].z_index,
                hits[0].sub_division_level,
            ))
        } else {
            None
        }
    }

    pub fn find_by_index(&self, x: i32, y: i32, z: i32, level: u32) -> Option<&Ocnode> {
        if level == self.sub_division_level {
            if self.x_index == x && self.y_index == y && self.z_index == z {
                return Some(self);
            } else {
                return None;
            }
        } else {
            if x >= self.x_index
                && (x <= self.x_index + self.resolution(self.sub_division_level) as i32)
                && y >= self.y_index
                && (y <= self.y_index + self.resolution(self.sub_division_level) as i32)
                && z >= self.z_index
                && (z <= self.z_index + self.resolution(self.sub_division_level) as i32)
            {
                if self.has_children {
                    let squirts = self.children.each_ref();

                    for node_opt in squirts {
                        match node_opt {
                            None => {
                                log::debug!("Should not get here")
                            }
                            Some(node) => {
                                let child = node.find_by_index(x, y, z, level);
                                if child.is_some() {
                                    return child;
                                }
                            }
                        };
                    }
                    return None;
                }
            }
            return None;
        }
    }

    pub fn uniform(&self, compare: &Ocnode) -> bool {
        let compare_color = compare.color;
        let compare_fluid = compare.fluid;
        let compare_noise = compare.noise;
        !(compare_color[0] != self.color[0]
            || compare_color[1] != self.color[1]
            || compare_color[2] != self.color[2]
            || compare_color[3] != self.color[3]
            || compare_fluid != self.fluid
            || compare_noise != self.noise)
    }

    pub fn bottom_occluded(&self, root: &Ocnode) -> bool {
        let maybe_bottom = root.find_by_index(
            self.x_index,
            self.y_index - self.resolution(self.sub_division_level) as i32,
            self.z_index,
            self.sub_division_level,
        );
        if maybe_bottom.is_some() {
            let bottom = maybe_bottom.unwrap();
            if bottom.active {
                return self.uniform(bottom);
            }
        } else {
            print!("Bottom occlusion check failed");
        }
        false
    }

    pub fn left_occluded(&self, root: &Ocnode) -> bool {
        let maybe_left = root.find_by_index(
            self.x_index - self.resolution(self.sub_division_level) as i32,
            self.y_index,
            self.z_index,
            self.sub_division_level,
        );
        if maybe_left.is_some() {
            let left = maybe_left.unwrap();
            if left.active {
                return self.uniform(left);
            }
        } else {
            print!("Left occlusion check failed");
        }
        false
    }

    pub fn right_occluded(&self, root: &Ocnode) -> bool {
        let maybe_right = root.find_by_index(
            self.x_index + self.resolution(self.sub_division_level) as i32,
            self.y_index,
            self.z_index,
            self.sub_division_level,
        );
        if maybe_right.is_some() {
            let right = maybe_right.unwrap();
            if right.active {
                return self.uniform(right);
            }
        } else {
            print!("Right occlusion check failed");
        }
        false
    }

    pub fn front_occluded(&self, root: &Ocnode) -> bool {
        let maybe_front = root.find_by_index(
            self.x_index,
            self.y_index,
            self.z_index - self.resolution(self.sub_division_level) as i32,
            self.sub_division_level,
        );
        if maybe_front.is_some() {
            let front = maybe_front.unwrap();
            if front.active {
                return self.uniform(front);
            }
        } else {
            print!("Front occlusion check failed");
        }
        false
    }

    pub fn back_occluded(&self, root: &Ocnode) -> bool {
        let maybe_back = root.find_by_index(
            self.x_index,
            self.y_index,
            self.z_index + self.resolution(self.sub_division_level) as i32,
            self.sub_division_level,
        );
        if maybe_back.is_some() {
            let back = maybe_back.unwrap();
            if back.active {
                return self.uniform(back);
            }
        } else {
            print!("Back occlusion check failed");
        }
        false
    }

    pub fn top_occluded(&self, root: &Ocnode) -> bool {
        let maybe_top = root.find_by_index(
            self.x_index,
            self.y_index + self.resolution(self.sub_division_level) as i32,
            self.z_index,
            self.sub_division_level,
        );
        if maybe_top.is_some() {
            let top = maybe_top.unwrap();
            if top.active {
                return self.uniform(top);
            }
        } else {
            print!("Top occlusion check failed");
        }
        false
    }

    pub fn find_mut_by_index(&mut self, x: i32, y: i32, z: i32, level: u32) -> Option<&mut Ocnode> {
        if level == self.sub_division_level {
            if self.x_index == x && self.y_index == y && self.z_index == z {
                return Some(self);
            } else {
                return None;
            }
        } else {
            if x >= self.x_index
                && (x <= self.x_index + self.resolution(self.sub_division_level) as i32)
                && y >= self.y_index
                && (y <= self.y_index + self.resolution(self.sub_division_level) as i32)
                && z >= self.z_index
                && (z <= self.z_index + self.resolution(self.sub_division_level) as i32)
            {
                if self.has_children {
                    let squirts = self.children.each_mut();

                    for node_opt in squirts {
                        match node_opt {
                            None => {
                                log::debug!("Should not get here")
                            }
                            Some(node) => {
                                let child = node.find_mut_by_index(x, y, z, level);
                                if child.is_some() {
                                    return child;
                                }
                            }
                        };
                    }
                    return None;
                }
            }
            return None;
        }
    }

    /// Return the coordinate range. The actual positions go from -range to +range
    pub const fn range() -> i32 {
        2i32.pow(LEVELS - 1) / 2
    }

    /// Calculate the width of a cube at this subdivision level
    pub fn resolution(&self, sub_division_level: u32) -> u32 {
        let power = LEVELS.checked_sub(sub_division_level).expect("");
        2u32.pow(power)
    }

    /// Get the list of active cubes including this one and all it's children.
    pub fn active_nodes(&self) -> Vec<Ocnode> {
        let mut found: Vec<Ocnode> = vec![];

        if self.active {
            found.push(self.clone());
        }
        if self.has_children {
            let squirts = self.children.each_ref();

            for node_opt in squirts {
                match node_opt {
                    None => {
                        log::debug!("Should not get here")
                    }
                    Some(node) => {
                        found.extend(node.active_nodes());
                    }
                };
            }
        }

        found
    }

    /// Set this cube and all it's children to hidden.
    pub fn clear(&mut self) {
        self.active = false;

        let squirts = self.children.each_mut();

        for node_opt in squirts {
            match node_opt {
                None => {}
                Some(squirt) => {
                    squirt.clear();
                }
            };
        }
    }

    /// Used when restoring from serial form.
    pub fn apply(&mut self, node: &Ocnode) {
        let found_opt = self.find_mut_by_index(
            node.x_index,
            node.y_index,
            node.z_index,
            node.sub_division_level,
        );

        if found_opt.is_some() {
            let found = found_opt.unwrap();
            // We got a match. Apply it.
            found.active = node.active;
            found.color = node.color;
            found.fluid = node.fluid;
            found.noise = node.noise;
            found.back_occluded_calculated = node.back_occluded_calculated;
            found.front_occluded_calculated = node.front_occluded_calculated;
            found.top_occluded_calculated = node.top_occluded_calculated;
            found.bottom_occluded_calculated = node.bottom_occluded_calculated;
            found.left_occluded_calculated = node.left_occluded_calculated;
            found.right_occluded_calculated = node.right_occluded_calculated;
        }
    }

    /// Determine the distance between this cube and the camera.
    fn _depth(&self, camera: [f32; 3]) -> f32 {
        let half = self.resolution(self.sub_division_level) as f32 / 2.0;
        ((self.x_index as f32 + half - camera[0]).powi(2)
            + (self.y_index as f32 + half - camera[1]).powi(2)
            + (self.z_index as f32 + half - camera[2]).powi(2))
        .sqrt()
    }

    /// Set the active state to match the combined active state of all children.
    pub fn optimize(&mut self, _camera_eye: [f32; 3]) {

        /*if self.has_children {
            // Optimize leaf first then move up the tree.
            let squirts = self.children.each_mut();
            for child in squirts {
                match child {
                    None => {}
                    Some(down) => {
                        down.optimize(camera_eye);
                    }
                }
            }
            let squirts = self.children.each_mut();
            let has_peg = squirts
                .into_iter()
                .any(|child| child.as_ref().expect("child").active);

            let squirts = self.children.each_mut();
            let has_hole = squirts
                .into_iter()
                .any(|child| !child.as_ref().expect("child").active);
            //let squirts = self.children.each_mut();
            let mut color = [0.0, 0.0, 0.0, 0.0];
            let mut fluid = 0;
            let mut noise = 0;
            for i in self.children.each_mut() {
                let node = i.as_ref().expect("child");
                if node.active {
                    color = node.color;
                    fluid = node.fluid;
                    noise = node.noise;
                }
            }

            let squirts = self.children.each_mut();
            let not_uniform_color = squirts.into_iter().any(|child| {
                let compare = child.as_ref().expect("child").color;
                let compare_fluid = child.as_ref().expect("child").fluid;
                let compare_noise = child.as_ref().expect("child").noise;
                compare[0] != color[0]
                    || compare[1] != color[1]
                    || compare[2] != color[2]
                    || compare[3] != color[3]
                    || compare_fluid != fluid
                    || compare_noise != noise
            });

            let res = LEVELS.checked_sub(self.sub_division_level).expect("");
            let depth = self.depth(camera_eye) / res as f32;
            let lod = 60.0;

            self.active = (has_peg && (depth > lod)) || (!has_hole && !not_uniform_color);
            for i in self.children.each_mut() {
                let node = i.as_ref().expect("child");
                if node.active {
                    self.color = node.color;
                    self.fluid = node.fluid;
                    self.noise = node.noise;
                }
            }
        }*/
    }

    /// Are all the nodes in the list of nodes active?
    pub fn all_voxels_active(&self, positions: &Vec<[i32; 3]>) -> bool {
        for position in positions {
            let found = self.find_by_index(position[0], position[1], position[2], LEVELS);
            if found.is_some() {
                if !found.unwrap().active {
                    return false;
                }
            } else {
                log::error!("position could not be found: {:?}", position);
            }
        }

        true
    }

    pub fn toggle_voxels(
        &mut self,
        positions: &Vec<[i32; 3]>,
        value: bool,
        color: [f32; 4],
        fluid: i32,
        noise: i32,
    ) {
        for position in positions {
            let maybe = self.find_mut_by_index(position[0], position[1], position[2], LEVELS);
            if maybe.is_some() {
                let actual = maybe.unwrap();
                actual.active = value;
                actual.color = color;
                actual.fluid = fluid;
                actual.noise = noise;
            }
        }
    }

    /// Generate a list of drawables from the active cubes in this one.
    pub fn drawables(&mut self) -> Vec<Cube> {
        if self.has_children {
            if self.active {
                let scale = self.resolution(self.sub_division_level) as f32;
                let mut cube = Cube::new();

                cube.color = self.color;
                cube.fluid = self.fluid;
                cube.noise = self.noise;
                cube.scale = scale;
                cube.smooth = true;

                cube.bottom_occluded = self.bottom_occluded_calculated;
                cube.left_occluded = self.left_occluded_calculated;
                cube.right_occluded = self.right_occluded_calculated;
                cube.front_occluded = self.front_occluded_calculated;
                cube.back_occluded = self.back_occluded_calculated;
                cube.top_occluded = self.top_occluded_calculated;
                cube.init();

                let x = self.x_index as f32 * (1.0);
                let y = self.y_index as f32 * (1.0);
                let z = self.z_index as f32 * (1.0);
                cube.translate([x, y, z]);

                vec![cube]
            } else {
                let mut child_cubes: Vec<Cube> = vec![];
                let squirts = self.children.each_mut();

                for node_opt in squirts {
                    match node_opt {
                        None => {}
                        Some(node) => {
                            let mut cube = node.drawables();

                            child_cubes.append(&mut cube);
                        }
                    };
                }
                child_cubes
            }
        } else if self.active {
            let scale = 1.0;
            let mut cube = Cube::new();

            cube.color = self.color;
            cube.fluid = self.fluid;
            cube.noise = self.noise;
            cube.scale = scale;
            cube.smooth = true;

            cube.bottom_occluded = self.bottom_occluded_calculated;
            cube.left_occluded = self.left_occluded_calculated;
            cube.right_occluded = self.right_occluded_calculated;
            cube.front_occluded = self.front_occluded_calculated;
            cube.back_occluded = self.back_occluded_calculated;
            cube.top_occluded = self.top_occluded_calculated;
            cube.init();

            let x = self.x_index as f32 * (scale);
            let y = self.y_index as f32 * (scale);
            let z = self.z_index as f32 * (scale);

            cube.translate([x, y, z]);

            vec![cube]
        } else {
            vec![]
        }
    }

    pub fn recalculate_occlusion(&mut self, root: &Ocnode) {
        if self.active {
            self.front_occluded_calculated = self.front_occluded(root);
            self.back_occluded_calculated = self.back_occluded(root);
            self.top_occluded_calculated = self.top_occluded(root);
            self.bottom_occluded_calculated = self.bottom_occluded(root);
            self.left_occluded_calculated = self.left_occluded(root);
            self.right_occluded_calculated = self.right_occluded(root);
        }
        if self.has_children {
            let squirts = self.children.each_mut();
            for node_opt in squirts {
                match node_opt {
                    None => {}
                    Some(node) => {
                        node.recalculate_occlusion(root);
                    }
                };
            }
        }
    }

    /// Create smaller children cubes from this outer cube.
    pub fn decimate(&mut self, sub_division_level: u32) {
        if sub_division_level - 1 > 0 {
            self.subdivide();

            let squirts = self.children.each_mut();

            for node_opt in squirts {
                match node_opt {
                    None => {
                        log::debug!("Should not get here")
                    }
                    Some(node) => {
                        node.decimate(sub_division_level - 1);
                    }
                };
            }
        }
    }

    /// Used by the decimate function to create smaller cubes.
    pub fn subdivide(&mut self) {
        self.has_children = true;

        self.children[0] = Some(Box::new(Ocnode {
            x_index: self.x_index,
            y_index: self.y_index,
            z_index: self.z_index,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
            noise: self.noise,
            back_occluded_calculated: false,
            top_occluded_calculated: false,
            bottom_occluded_calculated: false,
            left_occluded_calculated: false,
            right_occluded_calculated: false,
            front_occluded_calculated: false,
        }));

        self.children[1] = Some(Box::new(Ocnode {
            x_index: self.x_index + self.resolution(self.sub_division_level + 1) as i32,
            y_index: self.y_index,
            z_index: self.z_index,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
            noise: self.noise,
            back_occluded_calculated: false,
            top_occluded_calculated: false,
            bottom_occluded_calculated: false,
            left_occluded_calculated: false,
            right_occluded_calculated: false,
            front_occluded_calculated: false,
        }));
        self.children[2] = Some(Box::new(Ocnode {
            x_index: self.x_index,
            y_index: self.y_index + self.resolution(self.sub_division_level + 1) as i32,
            z_index: self.z_index,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
            noise: self.noise,
            back_occluded_calculated: false,
            top_occluded_calculated: false,
            bottom_occluded_calculated: false,
            left_occluded_calculated: false,
            right_occluded_calculated: false,
            front_occluded_calculated: false,
        }));
        self.children[3] = Some(Box::new(Ocnode {
            x_index: self.x_index,
            y_index: self.y_index,
            z_index: self.z_index + self.resolution(self.sub_division_level + 1) as i32,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
            noise: self.noise,
            back_occluded_calculated: false,
            top_occluded_calculated: false,
            bottom_occluded_calculated: false,
            left_occluded_calculated: false,
            right_occluded_calculated: false,
            front_occluded_calculated: false,
        }));
        self.children[4] = Some(Box::new(Ocnode {
            x_index: self.x_index + self.resolution(self.sub_division_level + 1) as i32,
            y_index: self.y_index + self.resolution(self.sub_division_level + 1) as i32,
            z_index: self.z_index,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
            noise: self.noise,
            back_occluded_calculated: false,
            top_occluded_calculated: false,
            bottom_occluded_calculated: false,
            left_occluded_calculated: false,
            right_occluded_calculated: false,
            front_occluded_calculated: false,
        }));
        self.children[5] = Some(Box::new(Ocnode {
            x_index: self.x_index,
            y_index: self.y_index + self.resolution(self.sub_division_level + 1) as i32,
            z_index: self.z_index + self.resolution(self.sub_division_level + 1) as i32,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
            noise: self.noise,
            back_occluded_calculated: false,
            top_occluded_calculated: false,
            bottom_occluded_calculated: false,
            left_occluded_calculated: false,
            right_occluded_calculated: false,
            front_occluded_calculated: false,
        }));
        self.children[6] = Some(Box::new(Ocnode {
            x_index: self.x_index + self.resolution(self.sub_division_level + 1) as i32,
            y_index: self.y_index,
            z_index: self.z_index + self.resolution(self.sub_division_level + 1) as i32,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
            noise: self.noise,
            back_occluded_calculated: false,
            top_occluded_calculated: false,
            bottom_occluded_calculated: false,
            left_occluded_calculated: false,
            right_occluded_calculated: false,
            front_occluded_calculated: false,
        }));
        self.children[7] = Some(Box::new(Ocnode {
            x_index: self.x_index + self.resolution(self.sub_division_level + 1) as i32,
            y_index: self.y_index + self.resolution(self.sub_division_level + 1) as i32,
            z_index: self.z_index + self.resolution(self.sub_division_level + 1) as i32,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
            noise: self.noise,
            back_occluded_calculated: false,
            top_occluded_calculated: false,
            bottom_occluded_calculated: false,
            left_occluded_calculated: false,
            right_occluded_calculated: false,
            front_occluded_calculated: false,
        }));
    }
}
