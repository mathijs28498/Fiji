use std::sync::Arc;

use nalgebra_glm::{Vec2, Vec4};
use vulkano::{
    buffer::{BufferUsage, ImmutableBuffer},
    device::Queue,
};

use crate::rendering::{
    data_types::Vertex,
    device_container::DeviceContainer,
    render_passes::circle_render_pass::{CirclePushConstants, CircleRenderPass},
};

#[derive(Clone)]
pub struct Circle {
    pub color: Vec4,
    pub position: Vec2,
    pub radius: f32,
    pub vertex_buffer: Option<Arc<ImmutableBuffer<[Vertex]>>>,
    pub index_buffer: Option<Arc<ImmutableBuffer<[u32]>>>,
}

impl Circle {
    pub fn new(color: Vec4, position: Vec2, radius: f32) -> Self {
        Self {
            color,
            position,
            radius,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut CircleRenderPass,
        device_container: &mut DeviceContainer,
    ) {
        let (vertex_buffer, index_buffer) = self.get_buffers(device_container.queue());
        render_pass.draw(
            device_container,
            vertex_buffer,
            index_buffer,
            CirclePushConstants::new(
                self.color.clone(),
                self.position.clone(),
                self.radius.clone(),
            ),
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
                        Vertex {
                            position: [-1., -1.],
                        },
                        Vertex {
                            position: [1., -1.],
                        },
                        Vertex {
                            position: [-1., 1.],
                        },
                        Vertex { position: [1., 1.] },
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
