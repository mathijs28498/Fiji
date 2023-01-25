use nalgebra_glm::{Vec3, Vec4};

#[derive(Clone, Debug)]
pub struct Background {
    pub color: Vec3,
}

impl Background {
    pub fn new_with_color(color: Vec3) -> Self {
        Self { color }
    }
    pub fn background_color(&self) -> [f32; 4] {
        [self.color.x, self.color.y, self.color.z, 1.]
    }
}
