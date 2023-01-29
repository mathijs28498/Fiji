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
pub struct Vertex2D {
    pub(crate) position: [f32; 2],
}
impl_vertex!(Vertex2D, position);

#[derive(Clone, Debug)]
pub struct BufferContainer2D {
    pub(crate) vertex_buffer: Arc<DeviceLocalBuffer<[Vertex2D]>>,
    pub(crate) index_buffer: Arc<DeviceLocalBuffer<[u32]>>,
}

pub fn create_buffers_2d(
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