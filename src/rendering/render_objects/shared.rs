use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use vulkano::{
    buffer::{BufferUsage, DeviceLocalBuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract},
    impl_vertex,
    sync::GpuFuture,
};

use crate::rendering::render_containers::device_container::DeviceContainer;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub(crate) struct Vertex2D {
    pub(crate) position: [f32; 2],
}
impl_vertex!(Vertex2D, position);

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub(crate) struct Vertex2DUv {
    pub(crate) position: [f32; 2],
    pub(crate) uvCoord: [f32; 2],
}
impl_vertex!(Vertex2DUv, position, uvCoord);

#[derive(Clone, Debug)]
pub(crate) struct BufferContainer2D {
    pub(crate) vertex_buffer: Arc<DeviceLocalBuffer<[Vertex2D]>>,
    pub(crate) index_buffer: Arc<DeviceLocalBuffer<[u32]>>,
}

#[derive(Clone, Debug)]
pub(crate) struct BufferContainer2DUv {
    pub(crate) vertex_buffer: Arc<DeviceLocalBuffer<[Vertex2DUv]>>,
    pub(crate) index_buffer: Arc<DeviceLocalBuffer<[u32]>>,
}

#[derive(Clone, Debug)]
pub(crate) struct BufferContainer3D {
    pub(crate) vertex_buffer: Arc<DeviceLocalBuffer<[Vertex3D]>>,
    pub(crate) index_buffer: Arc<DeviceLocalBuffer<[u32]>>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub(crate) struct Vertex3D {
    pub(crate) position: [f32; 3],
    pub(crate) normal: [f32; 3],
}
impl_vertex!(Vertex3D, position, normal);

pub(crate) fn create_buffers_2d(
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

pub(crate) fn create_buffers_2d_uv(
    device_container: &mut DeviceContainer,
    vertices: Vec<Vertex2DUv>,
    indices: Vec<u32>,
) -> BufferContainer2DUv {
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

    BufferContainer2DUv {
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
