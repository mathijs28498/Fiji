use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder, ClearColorImageInfo, ClearDepthStencilImageInfo,
        CommandBufferUsage,
    },
    sync::GpuFuture,
};

use nalgebra_glm::Vec3;

use crate::rendering::render_containers::device_container::DeviceContainer;
pub(crate) struct BackgroundRenderPass {}

impl BackgroundRenderPass {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn draw(&self, device_container: &mut DeviceContainer, color: &Vec3) {
        let mut builder = AutoCommandBufferBuilder::primary(
            device_container.command_buffer_allocator(),
            device_container.queue_family_index(),
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

        device_container.execute_command_buffer(builder.build().unwrap());
    }
}
