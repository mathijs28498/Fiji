use nalgebra_glm::Vec2;

#[derive(Debug)]
pub struct Camera2D {
    pub position: Vec2,
}

impl Camera2D {
    pub fn new_default() -> Self {
        Self {
            position: Vec2::new(0., 0.),
        }
    }
}
