use std::sync::Arc;

use nalgebra_glm::{Vec2, Vec4};
use vulkano::{
    buffer::{BufferUsage, ImmutableBuffer},
    device::Queue,
};

use crate::rendering::{
    data_types::{BufferContainer, Vertex},
    device_container::DeviceContainer,
    render_passes::{
        circle_render_pass::CircleRenderPass,
        poly_render_pass::{PolyPushConstants, PolyRenderPass},
    },
};

use super::Border;

#[derive(Clone)]
pub struct Polygon {
    pub color: Vec4,
    pub points: Vec<Vec2>,
    pub border: Option<Border>,
    buffers: Option<BufferContainer>,
}

impl Polygon {
    pub fn new(color: Vec4, points: Vec<Vec2>, border: Option<Border>) -> Self {
        Self {
            color,
            points,
            border,
            buffers: None,
        }
    }

    pub fn new_triangle(color: Vec4, points: [Vec2; 3], border: Option<Border>) -> Self {
        Self {
            color,
            points: points.into(),
            border,
            buffers: None,
        }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut PolyRenderPass,
        device_container: &mut DeviceContainer,
    ) {
        if let None = self.buffers {
            self.buffers = Some(BufferContainer {
                vertex_buffer: self.get_vertex_buffer(device_container.queue().clone()),
                index_buffer: self.get_index_buffer(device_container.queue().clone()),
            })
        }

        render_pass.draw(
            device_container,
            self.buffers.as_ref().unwrap(),
            PolyPushConstants::new(
                self.color.clone(),
                Vec2::new(0., 0.),
                Vec2::new(1., 1.),
                self.border.clone(),
            ),
        );
    }

    // TODO: Implement proper vertex buffer shit
    fn get_vertex_buffer(&self, queue: Arc<Queue>) -> Arc<ImmutableBuffer<[Vertex]>> {
        let mut vertices = Vec::new();
        for p in &self.points {
            vertices.push(Vertex {
                position: [p.x, p.y],
            })
        }

        return ImmutableBuffer::from_iter(vertices, BufferUsage::vertex_buffer(), queue)
            .unwrap()
            .0;
    }

    fn get_index_buffer(&self, queue: Arc<Queue>) -> Arc<ImmutableBuffer<[u32]>> {
        if self.points.len() == 3 {
            return ImmutableBuffer::from_iter(
                [0, 1, 2],
                BufferUsage::index_buffer(),
                queue.clone(),
            )
            .unwrap()
            .0;
        }
        ImmutableBuffer::from_iter(
            [0, 1, 2, 2, 1, 3],
            BufferUsage::index_buffer(),
            queue.clone(),
        )
        .unwrap()
        .0
    }
}
