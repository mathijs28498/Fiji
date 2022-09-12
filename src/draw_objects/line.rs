use std::sync::Arc;

use nalgebra_glm::{Vec2, Vec4};
use vulkano::{buffer::{ImmutableBuffer, BufferUsage}, device::Queue};

use crate::rendering::{render_passes::line_render_pass::{LineRenderPass, LinePushConstants}, data_types::Vertex, device_container::DeviceContainer};

#[derive(Clone)]
pub struct Line {
    pub color: Vec4,
    pub p0: Vec2,
    pub p1: Vec2,
    pub thickness: u32,
}

impl Line {
    pub fn new(color: Vec4, p0: Vec2, p1: Vec2, thickness: u32) -> Self {
        Self {
            color,
            p0,
            p1,
            thickness,
        }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut LineRenderPass,
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
                LinePushConstants::new(
                    self.color.clone(),
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
