use nalgebra_glm::{Vec2, Vec4};

use crate::{
    public::objects::{Border, DEFAULT_COLOR},
    rendering::{
        render_containers::device_container::DeviceContainer,
        render_passes::render_passes_2d::poly_render_pass::PolyRenderPass,
    },
};

use super::{DEFAULT_POSITION_2D, DEFAULT_SIZE_2D};

#[derive(Clone, Debug)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Vec4,
    pub border: Option<Border>,
}

impl Rect {
    pub fn new_default() -> Self {
        Self {
            color: DEFAULT_COLOR,
            position: DEFAULT_POSITION_2D,
            size: DEFAULT_SIZE_2D,
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

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }
}
