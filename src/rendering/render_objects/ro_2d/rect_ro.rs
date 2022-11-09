use crate::{
    public::objects::{camera::camera_2d::Camera2D, obj_2d::rect::Rect},
    rendering::{
        render_containers::device_container::DeviceContainer,
        render_objects::shared::{create_buffers_2d, BufferContainer2D, Vertex2D},
        render_passes::render_passes_2d::poly_render_pass::PolyRenderPass,
    },
};

#[derive(Clone)]
pub(crate) struct RectRenderObject {
    pub(crate) rect: Rect,
    buffers: BufferContainer2D,
}

impl RectRenderObject {
    pub(crate) fn new(rect: Rect, device_container: &mut DeviceContainer) -> Self {
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

        Self { rect, buffers }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut PolyRenderPass,
        device_container: &mut DeviceContainer,
        camera_2d: Option<&Camera2D>,
    ) {
        render_pass.draw(
            device_container,
            &self.buffers,
            PolyRenderPass::create_push_constants(
                self.rect.color.clone(),
                self.rect.position.clone(),
                self.rect.size.clone(),
                self.rect.border.clone(),
                camera_2d,
            ),
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
