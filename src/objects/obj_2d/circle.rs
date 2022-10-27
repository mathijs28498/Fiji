use std::sync::Arc;

use nalgebra_glm::{Vec2, Vec4};
use vulkano::{
    buffer::{BufferUsage, ImmutableBuffer},
    device::Queue,
};

use crate::{
    objects::Border,
    rendering::{
        data_types::Vertex2D,
        device_container::DeviceContainer,
        render_passes::circle_render_pass::{CirclePushConstants, CircleRenderPass},
    },
};

static mut VERTEX_BUFFER: Option<Arc<ImmutableBuffer<[Vertex2D]>>> = None;
static mut INDEX_BUFFER: Option<Arc<ImmutableBuffer<[u32]>>> = None;

#[derive(Clone)]
pub struct Circle {
    pub color: Vec4,
    pub position: Vec2,
    pub radius: f32,
    pub border: Option<Border>,
}

impl Circle {
    pub fn new(color: Vec4, position: Vec2, radius: f32, border: Option<Border>) -> Self {
        Self {
            color,
            position,
            radius,
            border,
        }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut CircleRenderPass,
        device_container: &mut DeviceContainer,
    ) {
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
                CirclePushConstants::new(
                    self.color.clone(),
                    self.position.clone(),
                    self.radius.clone(),
                    self.border.clone(),
                ),
            );
        }
    }

    fn get_vertex_buffer(queue: Arc<Queue>) -> Arc<ImmutableBuffer<[Vertex2D]>> {
        ImmutableBuffer::from_iter(
            [
                Vertex2D {
                    position: [-1., -1.],
                },
                Vertex2D {
                    position: [1., -1.],
                },
                Vertex2D {
                    position: [-1., 1.],
                },
                Vertex2D { position: [1., 1.] },
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
