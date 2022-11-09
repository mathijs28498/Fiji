use crate::{
    public::objects::obj_2d::line::Line,
    rendering::{
        render_containers::device_container::DeviceContainer,
        render_objects::shared::{create_buffers_2d, BufferContainer2D, Vertex2D},
        render_passes::render_passes_2d::line_render_pass::LineRenderPass,
    },
};

#[derive(Clone)]
pub(crate) struct LineRenderObject {
    pub(crate) line: Line,
    buffers: BufferContainer2D,
}

impl LineRenderObject {
    pub(crate) fn new(line: Line, device_container: &mut DeviceContainer) -> Self {
        static mut BUFFERS: Option<BufferContainer2D> = None;
        let buffers;

        // Unsafe is used to change these static values.
        // This is definitely safe, even thought the compiler can't verify.
        unsafe {
            if let None = BUFFERS {
                BUFFERS = Some(Self::create_buffers(device_container));
            }
            buffers = BUFFERS.as_ref().unwrap().clone();
        }
        Self { line, buffers }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut LineRenderPass,
        device_container: &mut DeviceContainer,
    ) {
        render_pass.draw(
            device_container,
            &self.buffers,
            LineRenderPass::create_push_constants(self.line.color.clone()),
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
