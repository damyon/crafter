use crate::vertex::Vertex;
use glium::index::PrimitiveType;

/// Drawable objects can provide whats need to render themselves in WebGL.
pub trait Drawable {
    fn init(&mut self);
    fn count_vertices(&self) -> u16;
    fn translation(&self) -> &[f32; 3];
    fn rotation(&self) -> &[f32; 3];
    fn translate(&mut self, amount: [f32; 3]);
    fn rotate(&mut self, amount: [f32; 3]);
    fn vertices(&self) -> Vec<Vertex>;
    fn primitive_type(&self) -> PrimitiveType;
    fn color(&self) -> &[f32; 4];
    fn depth(&self, camera: [f32; 3]) -> f32;
    fn fluid(&self) -> i32;
    fn noise(&self) -> i32;
}
