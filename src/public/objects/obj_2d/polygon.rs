use nalgebra_glm::{Vec2, Vec4};

use crate::public::objects::{Border, DEFAULT_COLOR};

#[derive(Clone, Debug)]
pub struct Polygon {
    pub color: Vec4,
    pub points: Vec<Vec2>,
    pub border: Option<Border>,
}

impl Polygon {
    pub fn new_with_points(points: Vec<Vec2>) -> Self {
        Self {
            color: DEFAULT_COLOR,
            points,
            border: None,
        }
    }

    pub fn new_triangle(points: [Vec2; 3]) -> Self {
        Self {
            color: DEFAULT_COLOR,
            points: points.into(),
            border: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }
}
