use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use vulkano::{impl_vertex, buffer::DeviceLocalBuffer};

#[derive(Clone, Debug)]
pub(crate) struct BufferContainer2D {
    pub(crate) vertex_buffer: Arc<DeviceLocalBuffer<[Vertex2D]>>,
    pub(crate) index_buffer: Arc<DeviceLocalBuffer<[u32]>>,
}

#[derive(Clone, Debug)]
pub(crate) struct BufferContainer3D {
    pub(crate) vertex_buffer: Arc<DeviceLocalBuffer<[Vertex3D]>>,
    pub(crate) index_buffer: Arc<DeviceLocalBuffer<[u32]>>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Vertex2D {
    pub(crate) position: [f32; 2],
}
impl_vertex!(Vertex2D, position);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Vertex3D {
    pub(crate) position: [f32; 3],
    pub(crate) normal: [f32; 3],
}
impl_vertex!(Vertex3D, position, normal);