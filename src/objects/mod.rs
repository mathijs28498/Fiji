pub mod background;
pub mod obj_2d;
pub mod obj_3d;
pub mod camera;

use nalgebra_glm::Vec4;

use self::{
    background::Background,
    obj_2d::{circle::Circle, line::Line, polygon::Polygon, rect::Rect},
    obj_3d::block::{Block},
};

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

pub enum DrawObject3D {
    BlockObject(Block),
}

pub enum DrawObject2D {
    RectObject(Rect),
    CircleObject(Circle),
    LineObject(Line),
    PolyObject(Polygon),
}
