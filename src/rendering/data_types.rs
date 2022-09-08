use bytemuck::{Pod, Zeroable};
use vulkano::impl_vertex;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 2],
}
impl_vertex!(Vertex, position);