use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

// you must pass the list of members to the macro
implement_vertex!(Vertex, position, normal);
