use nalgebra_glm::{Vec2, Vec4};

use crate::{
    public::objects::DEFAULT_COLOR,
    rendering::{
        render_containers::device_container::DeviceContainer,
        render_passes::render_passes_2d::line_render_pass::LineRenderPass,
    },
};

#[derive(Clone, Debug)]
pub struct Line {
    pub color: Vec4,
    pub p0: Vec2,
    pub p1: Vec2,
    pub thickness: u32,
}

impl Line {
    pub fn new_with_points(p0: Vec2, p1: Vec2) -> Self {
        Self {
            color: DEFAULT_COLOR,
            p0,
            p1,
            thickness: 1,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_thickness(mut self, thickness: u32) -> Self {
        self.thickness = thickness;
        self
    }
}
