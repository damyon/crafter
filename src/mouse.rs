use nalgebra::Point2;

/// A reference to the state of the mouse.
pub struct Mouse {
    /// Remember the last position so we can find direction - e.g. drag.
    pub last_position: Point2<f32>,
    /// The button state of the mouse.
    pub is_pressed: bool,
}

impl Mouse {
    /// Create a new mouse struct.
    pub const fn new() -> Mouse {
        Mouse {
            last_position: Point2::new(0.0, 0.0),
            is_pressed: false,
        }
    }
}
