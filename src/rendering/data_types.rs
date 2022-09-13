use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use vulkano::{impl_vertex, buffer::ImmutableBuffer};

#[derive(Clone)]
pub(crate) struct BufferContainer {
    pub(crate) vertex_buffer: Arc<ImmutableBuffer<[Vertex]>>,
    pub(crate) index_buffer: Arc<ImmutableBuffer<[u32]>>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Vertex {
    pub(crate) position: [f32; 2],
}
impl_vertex!(Vertex, position);