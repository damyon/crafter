use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Material {
    pub color: [i32; 4],
    pub noise: i32,
    pub noise_x: i32,
    pub noise_y: i32,
    pub noise_z: i32,
    pub fluid: i32,
}

impl Material {
    pub fn new(
        color: [f32; 4],
        noise: i32,
        noise_x: i32,
        noise_y: i32,
        noise_z: i32,
        fluid: i32,
    ) -> Self {
        Material {
            color: [
                Material::downscale_color(color[0]),
                Material::downscale_color(color[1]),
                Material::downscale_color(color[2]),
                Material::downscale_color(color[3]),
            ],
            noise,
            noise_x,
            noise_y,
            noise_z,
            fluid,
        }
    }

    pub fn downscale_color(color: f32) -> i32 {
        (color * 255.0) as i32
    }

    pub fn upscale_color(&self) -> [f32; 4] {
        [
            self.color[0] as f32 / 255.0,
            self.color[1] as f32 / 255.0,
            self.color[2] as f32 / 255.0,
            self.color[3] as f32 / 255.0,
        ]
    }
}
