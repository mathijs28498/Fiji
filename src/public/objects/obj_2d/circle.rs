use nalgebra_glm::{Vec2, Vec4};

use crate::public::objects::{Border, DEFAULT_COLOR};

use super::DEFAULT_POSITION_2D;

#[derive(Clone, Debug)]
pub struct Circle {
    pub color: Vec4,
    pub position: Vec2,
    pub radius: f32,
    pub border: Option<Border>,
}

impl Circle {
    pub fn new_default() -> Self {
        Self {
            color: DEFAULT_COLOR,
            position: DEFAULT_POSITION_2D,
            radius: 10.,
            border: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }
}
