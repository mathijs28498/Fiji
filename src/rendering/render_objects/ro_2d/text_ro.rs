use crate::{
    public::objects::{camera::camera_2d::Camera2D, obj_2d::text::Text},
    rendering::{
        pipelines::pipelines_2d::text_pipeline::TextPipeline,
        render_containers::device_container::DeviceContainer,
    },
};

#[derive(Clone)]
pub(crate) struct TextRenderObject {
    text: Text,
}

impl TextRenderObject {
    pub(crate) fn new(text: Text, device_container: &mut DeviceContainer) -> Self {
        Self { text }
    }

    pub(crate) fn draw(&mut self, text_pipeline: &mut TextPipeline, camera_2d: Option<&Camera2D>) {}
}
