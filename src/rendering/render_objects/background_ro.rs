use crate::{
    public::objects::background::Background,
    rendering::{
        render_containers::device_container::DeviceContainer,
        pipelines::background_pipeline::BackgroundRenderPass,
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
        pipeline: &mut BackgroundRenderPass,
        device_container: &mut DeviceContainer,
    ) {
        pipeline.draw(device_container, &self.background.color);
    }
}
