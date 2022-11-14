use nalgebra_glm::{Vec2, Vec4};

use crate::public::objects::DEFAULT_COLOR;

use super::{DEFAULT_POSITION_2D, DEFAULT_FONT};

#[derive(Clone, Debug)]
pub enum TextFont {
    ComicSans,
    Roboto
}

#[derive(Clone, Debug)]
pub struct Text {
    pub text: String,
    pub color: Vec4,
    pub position: Vec2,
    pub font: TextFont,
}

impl Text {
    pub fn new_with_text(text: &str) -> Self {
        Self {
            text: text.to_string(),
            color: DEFAULT_COLOR,
            position: DEFAULT_POSITION_2D,
            font: DEFAULT_FONT,
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

    pub fn with_font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }
}
