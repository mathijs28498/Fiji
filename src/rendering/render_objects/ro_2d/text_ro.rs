use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract},
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    format::Format,
    image::{view::ImageView, ImageDimensions, ImmutableImage},
    pipeline::Pipeline,
    sync::GpuFuture,
};

use crate::{
    public::objects::{camera::camera_2d::Camera2D, obj_2d::text::Text},
    rendering::{
        pipelines::pipelines_2d::text_pipeline::{text_fs, TextPipeline},
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

    pub(crate) fn draw(
        &mut self,
        text_pipeline: &mut TextPipeline,
        device_container: &mut DeviceContainer,
        camera_2d: Option<&Camera2D>,
    ) {
        let sets = self
            .text
            .text
            .chars()
            .map(|c| text_pipeline.get_or_create_set(device_container, c, &self.text.font))
            .collect::<Vec<_>>();

        text_pipeline.draw(
            device_container,
            text_fs::ty::Constants {
                resolution: device_container.resolution(),
                position: self.text.position.as_ref().clone(),
                color: self.text.color.as_ref().clone(),
            },
            sets,
        );
    }
}
