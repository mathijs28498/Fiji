use crate::{
    public::objects::{camera::camera_3d::Camera3D, obj_3d::block::Block},
    rendering::{
        render_containers::device_container::DeviceContainer,
        render_objects::shared::{create_buffers_3d, BufferContainer3D, Vertex3D},
        render_passes::render_passes_3d::block_render_pass::BlockRenderPass,
    },
};

#[derive(Clone)]
pub(crate) struct BlockRenderObject {
    pub(crate) block: Block,
    buffers: BufferContainer3D,
}

impl BlockRenderObject {
    pub(crate) fn new(block: Block, device_container: &mut DeviceContainer) -> Self {
        static mut BUFFERS: Option<BufferContainer3D> = None;
        let buffers;

        // Unsafe is used to change these static values.
        // This is definitely safe, even thought the compiler can't verify.
        unsafe {
            if let None = BUFFERS {
                BUFFERS = Some(Self::create_buffers(device_container));
            }
            buffers = BUFFERS.as_ref().unwrap().clone();
        }

        Self { block, buffers }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut BlockRenderPass,
        device_container: &mut DeviceContainer,
        camera: &Camera3D,
    ) {
        render_pass.draw(
            device_container,
            &self.buffers,
            BlockRenderPass::create_push_constants(
                self.block.color.clone(),
                self.block.position.clone(),
                &self.block.size,
                self.block.rotation.clone(),
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
