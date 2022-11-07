use nalgebra_glm::{Vec2, Vec4};

use crate::{
    objects::help_functions::create_buffers_2d,
    rendering::{
        data_types::{BufferContainer2D, Vertex2D},
        device_container::DeviceContainer,
        render_passes::line_render_pass::{LinePushConstants, LineRenderPass},
    },
};

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

        //TODO: Change pipeline to use buffers instead of vertex_buffer/index_buffer
        render_pass.draw(
            device_container,
            buffers.vertex_buffer.clone(),
            buffers.index_buffer.clone(),
            LineRenderPass::create_push_constants(self.color.clone()),
        );
    }

    fn create_buffers(device_container: &mut DeviceContainer) -> BufferContainer2D {
        let vertices = vec![
            Vertex2D {
                position: [-0.5, -0.5],
            },
            Vertex2D {
                position: [0.5, -0.5],
            },
            Vertex2D {
                position: [-0.5, 0.5],
            },
            Vertex2D {
                position: [0.5, 0.5],
            },
        ];

        let indices = vec![0, 1, 2, 2, 1, 3];

        create_buffers_2d(device_container, vertices, indices)
    }
}
