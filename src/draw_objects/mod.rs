use std::sync::Arc;

use nalgebra_glm as glm;
use vulkano::{
    buffer::{BufferUsage, ImmutableBuffer},
    device::Queue,
};

use crate::rendering::{common::Context, data_types::Vertex};

#[derive(Clone)]
pub enum RenderPassType {
    Poly,
}

pub trait DrawObject {
    fn get_buffers(
        &mut self,
        queue: &Arc<Queue>,
    ) -> (Arc<ImmutableBuffer<[Vertex]>>, Arc<ImmutableBuffer<[u32]>>);

    fn render_pass_type(&self) -> RenderPassType;
}

pub struct Square {
    pub position: glm::Vec2,
    pub size: glm::Vec2,
    pub color: glm::Vec3,
    pub vertex_buffer: Option<Arc<ImmutableBuffer<[Vertex]>>>,
    pub index_buffer: Option<Arc<ImmutableBuffer<[u32]>>>,
    pub render_pass_type: RenderPassType,
}

impl Square {
    pub fn new(position: glm::Vec2, size: glm::Vec2, color: glm::Vec3) -> Self {
        Self {
            position,
            size,
            color,
            vertex_buffer: None,
            index_buffer: None,
            render_pass_type: RenderPassType::Poly,
        }
    }
}

impl DrawObject for Square {
    fn get_buffers(
        &mut self,
        queue: &Arc<Queue>,
    ) -> (Arc<ImmutableBuffer<[Vertex]>>, Arc<ImmutableBuffer<[u32]>>) {
        match &self.vertex_buffer {
            Some(vb) => (vb.clone(), self.index_buffer.as_ref().unwrap().clone()),
            None => {
                let (vertex_buffer, _) = ImmutableBuffer::from_iter(
                    [
                        Vertex { position: [0., 0.] },
                        Vertex { position: [1., 0.] },
                        Vertex { position: [0., 1.] },
                    ],
                    BufferUsage::vertex_buffer(),
                    queue.clone(),
                )
                .unwrap();
                let (index_buffer, _) = ImmutableBuffer::from_iter(
                    [0, 1, 2],
                    BufferUsage::index_buffer(),
                    queue.clone(),
                )
                .unwrap();
                self.vertex_buffer = Some(vertex_buffer.clone());
                self.index_buffer = Some(index_buffer.clone());
                (vertex_buffer, index_buffer)
            }
        }
    }

    fn render_pass_type(&self) -> RenderPassType {
        self.render_pass_type.clone()
    }
}

// pub struct ClearBackground {

// }

// impl DrawObject for ClearBackground {

// }
