use crate::{
    public::objects::background::Background,
    rendering::{
        render_containers::device_container::DeviceContainer,
        render_passes::background_render_pass::BackgroundRenderPass,
    },
};

pub(crate) struct BackgroundRenderObject {
    background: Background,
}

impl BackgroundRenderObject {
    pub(crate) fn new(background: Background) -> Self {
        Self { background }
    }

    pub(crate) fn draw(
        &self,
        render_pass: &mut BackgroundRenderPass,
        device_container: &mut DeviceContainer,
    ) {
        render_pass.draw(device_container, &self.background.color);
    }
}
