use nalgebra_glm::{Vec3, Vec4};
use vulkano::device::Queue;

use crate::{
    objects::{camera::camera_3d::Camera3D, help_functions::create_buffers_3d},
    rendering::{
        context::Context,
        data_types::{BufferContainer3D, Vertex3D},
        device_container::DeviceContainer,
        render_passes::block_render_pass::{BlockPushConstants, BlockRenderPass},
    },
};

#[derive(Clone, Debug)]
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
            rotation,
        }
    }

    pub fn new_default() -> Self {
        Self {
            color: Vec4::new(1., 1., 1., 1.),
            position: Vec3::new(0., 0., 0.),
            size: Vec3::new(1., 1., 1.),
            rotation: Vec3::new(0., 0., 0.),
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn with_size(mut self, size: Vec3) -> Self {
        self.size = size;
        self
    }

    pub fn with_rotation(mut self, rotation: Vec3) -> Self {
        self.rotation = rotation;
        self
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut BlockRenderPass,
        device_container: &mut DeviceContainer,
        camera: &Camera3D,
    ) {
        static mut BUFFERS: Option<BufferContainer3D> = None;
        let buffers;

        // Unsafe is used to change these static values.
        // This is definitely safe, even thought the compiler can't verify.
        unsafe {
            if let None = BUFFERS {
                BUFFERS = Some(Self::create_buffers(device_container));
            }
            buffers = BUFFERS.as_ref().unwrap();
        }

        render_pass.draw(
            device_container,
            buffers.vertex_buffer.clone(),
            buffers.index_buffer.clone(),
            BlockRenderPass::create_push_constants(
                self.color.clone(),
                self.position.clone(),
                &self.size,
                self.rotation.clone(),
                camera.get_view_matrix(),
                camera.position.clone(),
            ),
        );
    }

    fn create_buffers(device_container: &mut DeviceContainer) -> BufferContainer3D {
        let vertices = vec![
            // Front
            Vertex3D {
                position: [0.5, -0.5, 0.5],
                normal: [0., 0., 1.],
            },
            Vertex3D {
                position: [-0.5, -0.5, 0.5],
                normal: [0., 0., 1.],
            },
            Vertex3D {
                position: [0.5, 0.5, 0.5],
                normal: [0., 0., 1.],
            },
            Vertex3D {
                position: [-0.5, 0.5, 0.5],
                normal: [0., 0., 1.],
            },
            // Back
            Vertex3D {
                position: [-0.5, -0.5, -0.5],
                normal: [0., 0., -1.],
            },
            Vertex3D {
                position: [0.5, -0.5, -0.5],
                normal: [0., 0., -1.],
            },
            Vertex3D {
                position: [-0.5, 0.5, -0.5],
                normal: [0., 0., -1.],
            },
            Vertex3D {
                position: [0.5, 0.5, -0.5],
                normal: [0., 0., -1.],
            },
            //
            // Left
            Vertex3D {
                position: [0.5, -0.5, 0.5],
                normal: [1., 0., 0.],
            },
            Vertex3D {
                position: [0.5, 0.5, 0.5],
                normal: [1., 0., 0.],
            },
            Vertex3D {
                position: [0.5, -0.5, -0.5],
                normal: [1., 0., 0.],
            },
            Vertex3D {
                position: [0.5, 0.5, -0.5],
                normal: [1., 0., 0.],
            },
            // Right
            Vertex3D {
                position: [-0.5, -0.5, 0.5],
                normal: [-1., 0., 0.],
            },
            Vertex3D {
                position: [-0.5, -0.5, -0.5],
                normal: [-1., 0., 0.],
            },
            Vertex3D {
                position: [-0.5, 0.5, 0.5],
                normal: [-1., 0., 0.],
            },
            Vertex3D {
                position: [-0.5, 0.5, -0.5],
                normal: [-1., 0., 0.],
            },
            //
            // Top
            Vertex3D {
                position: [0.5, 0.5, 0.5],
                normal: [0., 1., 0.],
            },
            Vertex3D {
                position: [-0.5, 0.5, 0.5],
                normal: [0., 1., 0.],
            },
            Vertex3D {
                position: [0.5, 0.5, -0.5],
                normal: [0., 1., 0.],
            },
            Vertex3D {
                position: [-0.5, 0.5, -0.5],
                normal: [0., 1., 0.],
            },
            // Bottom
            Vertex3D {
                position: [0.5, -0.5, 0.5],
                normal: [0., -1., 0.],
            },
            Vertex3D {
                position: [0.5, -0.5, -0.5],
                normal: [0., -1., 0.],
            },
            Vertex3D {
                position: [-0.5, -0.5, 0.5],
                normal: [0., -1., 0.],
            },
            Vertex3D {
                position: [-0.5, -0.5, -0.5],
                normal: [0., -1., 0.],
            },
        ];

        let indices = vec![
            // Front
            0, 1, 2, //
            2, 1, 3, //
            // Back
            4, 5, 6, //
            6, 5, 7, //
            //
            // Left
            8, 9, 10, //
            10, 9, 11, //
            // Right
            12, 13, 14, //
            14, 13, 15, //
            //
            // Top
            16, 17, 18, //
            18, 17, 19, //
            // Bottom
            20, 21, 22, //
            22, 21, 23, //
        ];

        create_buffers_3d(device_container, vertices, indices)
    }
}
