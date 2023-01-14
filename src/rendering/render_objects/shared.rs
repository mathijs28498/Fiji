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
    let memory_allocator = device_container.memory_allocator();
    let mut builder = device_container.get_command_buffer_builder();

    let vertex_buffer = DeviceLocalBuffer::from_iter(
        memory_allocator.as_ref(),
        vertices,
        BufferUsage {
            vertex_buffer: true,
            ..Default::default()
        },
        &mut builder,
    )
    .unwrap();

    let index_buffer = DeviceLocalBuffer::from_iter(
        memory_allocator.as_ref(),
        indices,
        BufferUsage {
            index_buffer: true,
            ..Default::default()
        },
        &mut builder,
    )
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
    let memory_allocator = device_container.memory_allocator();
    let builder = device_container.get_command_buffer_builder();
    let vertex_buffer = DeviceLocalBuffer::from_iter(
        memory_allocator.as_ref(),
        vertices,
        BufferUsage {
            vertex_buffer: true,
            ..Default::default()
        },
        builder,
    )
    .unwrap();

    let index_buffer = DeviceLocalBuffer::from_iter(
        memory_allocator.as_ref(),
        indices,
        BufferUsage {
            index_buffer: true,
            ..Default::default()
        },
        builder,
    )
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
    let memory_allocator = device_container.memory_allocator();
    let mut builder = device_container.get_command_buffer_builder();

    let vertex_buffer = DeviceLocalBuffer::from_iter(
        memory_allocator.as_ref(),
        vertices,
        BufferUsage {
            vertex_buffer: true,
            ..Default::default()
        },
        builder,
    )
    .unwrap();

    let index_buffer = DeviceLocalBuffer::from_iter(
        memory_allocator.as_ref(),
        indices,
        BufferUsage {
            index_buffer: true,
            ..Default::default()
        },
        builder,
    )
    .unwrap();

    BufferContainer3D {
        vertex_buffer,
        index_buffer,
    }
}
