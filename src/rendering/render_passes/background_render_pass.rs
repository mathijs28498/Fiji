use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder, ClearColorImageInfo, ClearDepthStencilImageInfo,
        CommandBufferUsage,
    },
    sync::GpuFuture,
};

use nalgebra_glm::Vec3;

use crate::rendering::device_container::DeviceContainer;
pub(crate) struct BackgroundRenderPass {}

impl BackgroundRenderPass {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn draw(&self, device_container: &mut DeviceContainer, color: &Vec3) {
        let mut builder = AutoCommandBufferBuilder::primary(
            device_container.device().clone(),
            device_container.queue().family(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .clear_color_image(ClearColorImageInfo {
                clear_value: [color.x, color.y, color.z, 1.].into(),
                ..ClearColorImageInfo::image(device_container.current_image().clone())
            })
            .unwrap();

        builder
            .clear_depth_stencil_image(ClearDepthStencilImageInfo {
                clear_value: 1.0.into(),
                ..ClearDepthStencilImageInfo::image(device_container.depth_image().clone())
            })
            .unwrap();

        let command_buffer = builder.build().unwrap();

        device_container.previous_frame_end = Some(
            device_container
                .previous_frame_end
                .take()
                .unwrap()
                .then_execute(device_container.queue().clone(), command_buffer)
                .unwrap()
                .boxed(),
        );
    }
}
