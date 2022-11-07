use nalgebra_glm::{Vec2, Vec4};

use crate::{
    objects::{help_functions::create_buffers_2d, Border},
    rendering::{
        data_types::{BufferContainer2D, Vertex2D},
        device_container::DeviceContainer,
        render_passes::circle_render_pass::{CirclePushConstants, CircleRenderPass},
    },
};

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
        static mut BUFFERS: Option<BufferContainer2D> = None;
        let buffers;

        // Unsafe is used to change these static values.
        // This is definitely safe, even thought the compiler can't verify.
        unsafe {
            if let None = BUFFERS {
                BUFFERS = Some(Self::create_buffers(device_container));
            }
            buffers = BUFFERS.as_ref().unwrap();
        }

        // Unsafe is used to change these static values.
        // This is definitely safe, even thought the compiler can't verify.
        render_pass.draw(
            device_container,
            buffers.vertex_buffer.clone(),
            buffers.index_buffer.clone(),
            CircleRenderPass::create_push_constants(
                self.color.clone(),
                self.position.clone(),
                self.radius.clone(),
                self.border.clone(),
            ),
        );
    }

    fn create_buffers(device_container: &mut DeviceContainer) -> BufferContainer2D {
        let vertices = vec![
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
        ];

        let indices = vec![0, 1, 2, 2, 1, 3];

        create_buffers_2d(device_container, vertices, indices)
    }
}
