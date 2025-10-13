use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct ImageVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

implement_vertex!(ImageVertex, position, tex_coords);
