use nalgebra_glm::Vec3;

#[derive(Clone, Debug)]
pub struct Background {
    pub color: Vec3,
}

impl Background {
    pub fn new_with_color(color: Vec3) -> Self {
        Self { color }
    }
}
