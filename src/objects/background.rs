use nalgebra_glm::Vec3;

use crate::rendering::{
    device_container::DeviceContainer, render_passes::background_render_pass::BackgroundRenderPass,
};

#[derive(Clone, Debug)]
pub struct Background {
    pub color: Vec3,
}

impl Background {
    pub fn new_with_color(color: Vec3) -> Self {
        Self {
            color,
        }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &BackgroundRenderPass,
        device_container: &mut DeviceContainer,
    ) {
        render_pass.draw(device_container, &self.color);
    }
}
