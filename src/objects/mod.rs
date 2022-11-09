pub mod background;
pub mod camera;
pub mod obj_2d;
pub mod obj_3d;

use nalgebra_glm::Vec4;

use self::{
    obj_2d::{circle::Circle, line::Line, polygon::Polygon, rect::Rect},
    obj_3d::block::Block,
};

const DEFAULT_COLOR: Vec4 = Vec4::new(1., 1., 1., 1.);

#[derive(Clone, Debug)]
pub struct Border {
    pub color: Vec4,
    pub width: u32,
}

impl Border {
    pub fn new(color: Vec4, width: u32) -> Self {
        Border { color, width }
    }
}
