use std::sync::Arc;

use nalgebra_glm::{Vec3, Vec4};
use vulkano::{
    buffer::{BufferUsage, ImmutableBuffer},
    device::Queue,
};

use crate::{
    objects::Border,
    rendering::{
        data_types::{Vertex2D, Vertex3D},
        device_container::DeviceContainer,
        render_passes::{
            block_render_pass::{BlockPushConstants, BlockRenderPass},
            circle_render_pass::{CirclePushConstants, CircleRenderPass},
        },
    },
};

static mut VERTEX_BUFFER: Option<Arc<ImmutableBuffer<[Vertex3D]>>> = None;
static mut INDEX_BUFFER: Option<Arc<ImmutableBuffer<[u32]>>> = None;

#[derive(Clone)]
pub struct Block {
    pub color: Vec4,
    pub position: Vec3,
    pub size: Vec3,
}

impl Block {
    pub fn new(color: Vec4, position: Vec3, size: Vec3) -> Self {
        Self {
            color,
            position,
            size,
        }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut BlockRenderPass,
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
                BlockPushConstants::new(
                    self.color.clone(),
                    Vec4::new(self.position.x, self.position.y, self.position.z, 1.),
                    Vec4::new(self.size.x, self.size.y, self.size.z, 1.),
                ),
            );
        }
    }

    fn get_vertex_buffer(queue: Arc<Queue>) -> Arc<ImmutableBuffer<[Vertex3D]>> {
        ImmutableBuffer::from_iter(
            [
                Vertex3D {
                    position: [-0.5, -0.5, 0.5],
                },
                Vertex3D {
                    position: [0.5, -0.5, 0.5],
                },
                Vertex3D {
                    position: [-0.5, 0.5, 0.5],
                },
                Vertex3D {
                    position: [0.5, 0.5, 0.5],
                },
                Vertex3D {
                    position: [-0.5, -0.5, -0.5],
                },
                Vertex3D {
                    position: [0.5, -0.5, -0.5],
                },
                Vertex3D {
                    position: [-0.5, 0.5, -0.5],
                },
                Vertex3D {
                    position: [0.5, 0.5, -0.5],
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
