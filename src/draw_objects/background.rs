use nalgebra_glm as glm;

use crate::rendering::{
    device_container::DeviceContainer, render_passes::background_render_pass::BackgroundRenderPass,
};

#[derive(Clone)]
pub struct Background {
    pub color: glm::Vec3,
}

impl Background {
    pub fn new(color: glm::Vec3) -> Self {
        Self { color }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &BackgroundRenderPass,
        device_container: &mut DeviceContainer,
    ) {
        render_pass.draw(device_container, &self.color);
    }
}
