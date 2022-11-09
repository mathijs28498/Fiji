use crate::{
    public::objects::obj_2d::circle::Circle,
    rendering::{
        render_containers::device_container::DeviceContainer,
        render_objects::shared::{create_buffers_2d, BufferContainer2D, Vertex2D},
        render_passes::render_passes_2d::circle_render_pass::CircleRenderPass,
    },
};

#[derive(Clone)]
pub(crate) struct CircleRenderObject {
    pub(crate) circle: Circle,
    buffers: BufferContainer2D,
}

impl CircleRenderObject {
    pub(crate) fn new(circle: Circle, device_container: &mut DeviceContainer) -> Self {
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
        Self { circle, buffers }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut CircleRenderPass,
        device_container: &mut DeviceContainer,
    ) {
        // Unsafe is used to change these static values.
        // This is definitely safe, even thought the compiler can't verify.
        render_pass.draw(
            device_container,
            &self.buffers,
            CircleRenderPass::create_push_constants(
                self.circle.color.clone(),
                self.circle.position.clone(),
                self.circle.radius.clone(),
                self.circle.border.clone(),
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
