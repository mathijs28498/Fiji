use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use vulkano::{impl_vertex, buffer::ImmutableBuffer};

#[derive(Clone)]
pub(crate) struct BufferContainer {
    pub(crate) vertex_buffer: Arc<ImmutableBuffer<[Vertex2D]>>,
    pub(crate) index_buffer: Arc<ImmutableBuffer<[u32]>>,
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
}
impl_vertex!(Vertex3D, position);