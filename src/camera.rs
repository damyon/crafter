use na::Point3;
use nalgebra as na;

/// A camera contains 2 points, the location and the target
#[derive(Copy, Clone)]
pub struct Camera {
    /// This is the location of the camera
    pub eye: Point3<f32>,
    /// This is what it is looking at
    pub target: Point3<f32>,
}

impl Camera {
    /// Creates a camera at the starting position looking at the origin.
    pub const fn new() -> Camera {
        Camera {
            eye: Point3::new(38.0, 16.0, 40.0),
            target: Point3::new(0.0, 0.0, 0.0),
        }
    }
}
