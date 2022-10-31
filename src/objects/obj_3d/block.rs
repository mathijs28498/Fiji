use std::sync::Arc;

use nalgebra_glm::{Vec3, Vec4};
use vulkano::{
    buffer::{BufferUsage, ImmutableBuffer},
    device::Queue,
};

use crate::{
    objects::{camera::camera_3d::Camera3D, Border},
    rendering::{
        context::Context,
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
    pub rotation: Vec3,
}

impl Block {
    pub fn new(color: Vec4, position: Vec3, size: Vec3, rotation: Vec3) -> Self {
        Self {
            color,
            position,
            size,
            rotation
        }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut BlockRenderPass,
        device_container: &mut DeviceContainer,
        camera: &Camera3D,
    ) {
        // Unsafe is used to change these static values.
        // This is definitely safe, even thought the compiler can't verify.
        unsafe {
            if let None = VERTEX_BUFFER {
                VERTEX_BUFFER = Some(Self::get_vertex_buffer(device_container.queue().clone()));
                INDEX_BUFFER = Some(Self::get_index_buffer(device_container.queue().clone()));
            }
        }

        let pc = BlockPushConstants::new(
            self.color.clone(),
            self.position.clone(),
            &self.size,
            self.rotation.clone(),
            camera.get_view_matrix(),
        );

        // Unsafe is used to change these static values.
        // This is definitely safe, even thought the compiler can't verify.
        unsafe {
            render_pass.draw(
                device_container,
                VERTEX_BUFFER.as_ref().unwrap().clone(),
                INDEX_BUFFER.as_ref().unwrap().clone(),
                pc,
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

    //TODO: Add backface culling
    fn get_index_buffer(queue: Arc<Queue>) -> Arc<ImmutableBuffer<[u32]>> {
        ImmutableBuffer::from_iter(
            [
                // Front
                0, 1, 2, //
                2, 1, 3, //
                // Back
                4, 5, 6, //
                6, 5, 7, //
                //
                // Left
                0, 2, 4, //
                4, 2, 6, //
                // Right
                1, 5, 3, //
                3, 5, 7, //
                //
                // Top
                0, 5, 1, //
                4, 5, 0, //
                // Bottom
                2, 7, 6, //
                2, 3, 7, //
            ],
            BufferUsage::index_buffer(),
            queue.clone(),
        )
        .unwrap()
        .0
    }
}
