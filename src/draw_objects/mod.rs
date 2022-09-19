pub mod background;
pub mod circle;
pub mod rect;
pub mod polygon;
pub mod line;

use nalgebra_glm::Vec4;

use self::{background::Background, circle::Circle, rect::Rect, polygon::Polygon, line::Line};

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
    RectObject(Rect),
    CircleObject(Circle),
    LineObject(Line),
    PolyObject(Polygon),
    BackgroundObject(Background),
}
