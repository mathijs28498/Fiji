use std::sync::Arc;

use nalgebra_glm::{Vec2, Vec4};
use vulkano::{buffer::{BufferUsage, ImmutableBuffer}, device::Queue};

use crate::rendering::{device_container::DeviceContainer, render_passes::{circle_render_pass::CircleRenderPass, poly_render_pass::{PolyRenderPass, PolyPushConstants}}, data_types::Vertex};

use super::Border;

#[derive(Clone)]
pub struct Polygon {
    pub color: Vec4,
    pub points: Vec<Vec2>,
    pub border: Option<Border>,
}

impl Polygon {
    pub fn new(color: Vec4, points: Vec<Vec2>, border: Option<Border>) -> Self {
        Self {
            color,
            points,
            border,
        }
    }

    pub fn new_triangle(color: Vec4, points: [Vec2; 3], border: Option<Border>) -> Self {
        Self {
            color,
            points: points.into(),
            border,
        }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut PolyRenderPass,
        device_container: &mut DeviceContainer,
    ) {
        static mut VERTEX_BUFFER: Option<Arc<ImmutableBuffer<[Vertex]>>> = None;
        static mut INDEX_BUFFER: Option<Arc<ImmutableBuffer<[u32]>>> = None;

        // Unsafe is used to change these static values.
        // This is definitely safe, even thought the compiler can't verify.
        unsafe {
            if let None = VERTEX_BUFFER {
                VERTEX_BUFFER = Some(Self::get_vertex_buffer(device_container.queue().clone()));
                INDEX_BUFFER = Some(Self::get_index_buffer(device_container.queue().clone()));
            }
        }

        // Unsafe is used to change these static values.
        // This is definitely safe, even thought the compiler can't verify.
        unsafe {
            render_pass.draw(
                device_container,
                VERTEX_BUFFER.as_ref().unwrap().clone(),
                INDEX_BUFFER.as_ref().unwrap().clone(),
                PolyPushConstants::new(
                    self.color.clone(),
                    Vec2::new(0., 0.),
                    Vec2::new(1., 1.),
                    self.border.clone(),
                ),
            );
        }
    }

    // TODO: Implement proper vertex buffer shit
    fn get_vertex_buffer(queue: Arc<Queue>) -> Arc<ImmutableBuffer<[Vertex]>> {
        ImmutableBuffer::from_iter(
            [
                Vertex {
                    position: [-0.5, -0.5],
                },
                Vertex {
                    position: [0.5, -0.5],
                },
                Vertex {
                    position: [-0.5, 0.5],
                },
                Vertex {
                    position: [0.5, 0.5],
                },
            ],
            BufferUsage::vertex_buffer(),
            queue,
        )
        .unwrap()
        .0
    }

    fn get_index_buffer(queue: Arc<Queue>) -> Arc<ImmutableBuffer<[u32]>> {
        ImmutableBuffer::from_iter(
            [0, 1, 2, 2, 1, 3],
            BufferUsage::index_buffer(),
            queue.clone(),
        )
        .unwrap()
        .0
    }
}
