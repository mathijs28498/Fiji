use vulkano::{
    buffer::{BufferUsage, DeviceLocalBuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract},
    sync::GpuFuture,
};

use crate::rendering::{
    data_types::{BufferContainer2D, BufferContainer3D, Vertex2D, Vertex3D},
    device_container::DeviceContainer,
};

pub(super) fn create_buffers_2d(
    device_container: &mut DeviceContainer,
    vertices: Vec<Vertex2D>,
    indices: Vec<u32>,
) -> BufferContainer2D {
    let mut cb_builder = AutoCommandBufferBuilder::primary(
        device_container.command_buffer_allocator(),
        device_container.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    let vertex_buffer = DeviceLocalBuffer::from_iter(
        device_container.memory_allocator(),
        vertices,
        BufferUsage {
            vertex_buffer: true,
            ..Default::default()
        },
        &mut cb_builder,
    )
    .unwrap();

    let index_buffer = DeviceLocalBuffer::from_iter(
        device_container.memory_allocator(),
        indices,
        BufferUsage {
            index_buffer: true,
            ..Default::default()
        },
        &mut cb_builder,
    )
    .unwrap();

    let cb = cb_builder.build().unwrap();

    cb.execute(device_container.queue().clone())
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    BufferContainer2D {
        vertex_buffer,
        index_buffer,
    }
}

pub(super) fn create_buffers_3d(
    device_container: &mut DeviceContainer,
    vertices: Vec<Vertex3D>,
    indices: Vec<u32>,
) -> BufferContainer3D {
    let mut cb_builder = AutoCommandBufferBuilder::primary(
        device_container.command_buffer_allocator(),
        device_container.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    let vertex_buffer = DeviceLocalBuffer::from_iter(
        device_container.memory_allocator(),
        vertices,
        BufferUsage {
            vertex_buffer: true,
            ..Default::default()
        },
        &mut cb_builder,
    )
    .unwrap();

    let index_buffer = DeviceLocalBuffer::from_iter(
        device_container.memory_allocator(),
        indices,
        BufferUsage {
            index_buffer: true,
            ..Default::default()
        },
        &mut cb_builder,
    )
    .unwrap();

    let cb = cb_builder.build().unwrap();

    cb.execute(device_container.queue().clone())
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    BufferContainer3D {
        vertex_buffer,
        index_buffer,
    }
}
