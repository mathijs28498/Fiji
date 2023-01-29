pub mod background;
pub mod camera;
pub mod obj_2d;
pub mod shared;

use nalgebra_glm::Vec4;

use crate::Color;

const DEFAULT_COLOR: Color = Color::Rgb(1., 1., 1.);

#[derive(Clone, Debug)]
pub struct Border {
    pub color: Color,
    pub width: u32,
}

impl Border {
    pub fn new(color: Color, width: u32) -> Self {
        Border { color, width }
    }
}
