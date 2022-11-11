use crate::{
    public::objects::{camera::camera_2d::Camera2D, obj_2d::rect::Rect},
    rendering::{
        render_containers::device_container::DeviceContainer,
        render_objects::shared::{create_buffers_2d, BufferContainer2D, Vertex2D},
        render_passes::render_passes_2d::poly_render_pass::{PolyRenderPass, poly_fs},
    },
};

#[derive(Clone)]
pub(crate) struct RectRenderObject {
    rect: Rect,
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
            self.create_push_constants(device_container, camera_2d),
        );
    }

    #[allow(non_snake_case)]
    pub(crate) fn create_push_constants(
        &self,
        device_container: &DeviceContainer,
        camera_2d: Option<&Camera2D>,
    ) -> poly_fs::ty::Constants {
        let (borderColor, borderWidth) = match &self.rect.border {
            Some(border) => (border.color.as_ref().clone(), border.width),
            None => ([0.; 4], 0),
        };

        let cameraPos = match camera_2d {
            Some(camera_2d) => camera_2d.position.as_ref().clone(),
            None => [0.; 2],
        };

        poly_fs::ty::Constants {
            resolution: device_container.resolution(),
            position: self.rect.position.as_ref().clone(),
            color: self.rect.color.as_ref().clone(),
            borderColor,
            size: self.rect.size.as_ref().clone(),
            borderWidth,
            cameraPos,
        }
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
