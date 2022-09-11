use std::sync::Arc;

use nalgebra_glm as glm;
use vulkano::{
    buffer::{BufferUsage, ImmutableBuffer},
    device::Queue,
};

use crate::rendering::{
    data_types::Vertex, device_container::DeviceContainer,
    render_passes::poly_render_pass::{PolyRenderPass, PolyPushConstants},
};

#[derive(Clone)]
pub struct Square {
    pub position: glm::Vec2,
    pub size: glm::Vec2,
    pub color: glm::Vec4,
    pub vertex_buffer: Option<Arc<ImmutableBuffer<[Vertex]>>>,
    pub index_buffer: Option<Arc<ImmutableBuffer<[u32]>>>,
}

impl Square {
    pub fn new(color: glm::Vec4, position: glm::Vec2, size: glm::Vec2) -> Self {
        Self {
            color,
            position,
            size,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut PolyRenderPass,
        device_container: &mut DeviceContainer,
    ) {
        let (vertex_buffer, index_buffer) = self.get_buffers(device_container.queue());
        render_pass.draw(
            device_container,
            vertex_buffer,
            index_buffer,
            PolyPushConstants::new(self.color.clone(), self.position.clone(), self.size.clone()),
        );
    }

    fn get_buffers(
        &mut self,
        queue: &Arc<Queue>,
    ) -> (Arc<ImmutableBuffer<[Vertex]>>, Arc<ImmutableBuffer<[u32]>>) {
        match &self.vertex_buffer {
            Some(vb) => (vb.clone(), self.index_buffer.as_ref().unwrap().clone()),
            None => {
                let (vertex_buffer, _) = ImmutableBuffer::from_iter(
                    [
                        Vertex { position: [-0.5, -0.5] },
                        Vertex { position: [0.5, -0.5] },
                        Vertex { position: [-0.5, 0.5] },
                        Vertex { position: [0.5, 0.5] },
                    ],
                    BufferUsage::vertex_buffer(),
                    queue.clone(),
                )
                .unwrap();
                let (index_buffer, _) = ImmutableBuffer::from_iter(
                    [0, 1, 2, 2, 1, 3],
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
}
