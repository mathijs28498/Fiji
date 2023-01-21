use crate::{
    public::objects::obj_2d::circle::Circle,
    rendering::{
        pipelines::pipelines_2d::circle_pipeline::{circle_fs, CirclePipeline},
        render_containers::device_container::DeviceContainer,
        render_objects::shared::{create_buffers_2d, BufferContainer2D, Vertex2D},
    },
    Camera2D,
};

#[derive(Clone)]
pub(crate) struct CircleRenderObject {
    circle: Circle,
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
        pipeline: &mut CirclePipeline,
        device_container: &mut DeviceContainer,
        camera_2d: Option<&Camera2D>,
    ) {
        pipeline.draw(
            device_container,
            &self.buffers,
            self.create_push_constants(device_container, camera_2d),
        );
    }

    #[allow(non_snake_case)]
    fn create_push_constants(
        &self,
        device_container: &DeviceContainer,
        camera_2d: Option<&Camera2D>,
    ) -> circle_fs::ty::Constants {
        let (borderColor, borderWidth) = match &self.circle.border {
            Some(border) => (border.color.as_ref().clone(), border.width),
            None => ([0.; 4], 0),
        };

        let cameraPos = match camera_2d {
            Some(camera_2d) => camera_2d.position.as_ref().clone(),
            None => [0.; 2],
        };

        circle_fs::ty::Constants {
            resolution: device_container.resolution(),
            color: self.circle.color.as_ref().clone(),
            position: self.circle.position.as_ref().clone(),
            borderColor,
            borderWidth,
            radius: self.circle.radius,
            cameraPos,
        }
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
