use nalgebra_glm::{Vec2, Vec4};

use crate::public::objects::DEFAULT_COLOR;

use super::DEFAULT_POSITION_2D;

#[derive(Clone, Debug)]
pub struct Text {
    text: String,
    color: Vec4,
    position: Vec2,
}

impl Text {
    pub fn new_with_text(text: &str) -> Self {
        Self {
            text: text.to_string(),
            color: DEFAULT_COLOR,
            position: DEFAULT_POSITION_2D,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }
}
