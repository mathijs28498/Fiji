pub mod background;
pub mod circle;
pub mod square;

use nalgebra_glm::Vec4;

use self::{background::Background, circle::Circle, square::Square};

#[derive(Clone)]
pub struct Border {
    pub color: Vec4,
    pub width: u32,
}

impl Border {
    pub fn new(color: Vec4, width: u32) -> Self {
        Border { color, width }
    }
}

pub enum DrawObject {
    SquareObject(Square),
    BackgroundObject(Background),
    CircleObject(Circle),
}
