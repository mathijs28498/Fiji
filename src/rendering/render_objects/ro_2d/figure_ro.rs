use crate::{
    rendering::{
        pipelines::pipelines_2d::figure_pipeline::{figure_fs, FigurePipeline},
        render_containers::device_container::DeviceContainer,
        render_objects::shared::{create_buffers_2d, BufferContainer2D, Vertex2D, BufferContainer2DUv, Vertex2DUv, create_buffers_2d_uv},
    },
    Camera2D, Figure,
};

#[derive(Clone)]
pub(crate) struct FigureRenderObject {
    figure: Figure,
    buffers: BufferContainer2DUv,
}

impl FigureRenderObject {
    pub(crate) fn new(figure: Figure, device_container: &mut DeviceContainer) -> Self {
        static mut BUFFERS: Option<BufferContainer2DUv> = None;
        let buffers;

        // Unsafe is used to change these static values.
        // This is definitely safe, even thought the compiler can't verify.
        unsafe {
            if let None = BUFFERS {
                BUFFERS = Some(Self::create_buffers(device_container));
            }

            buffers = BUFFERS.as_ref().unwrap().clone();
        }

        Self { figure, buffers }
    }

    pub(crate) fn draw(
        &mut self,
        pipeline: &mut FigurePipeline,
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
    ) -> figure_fs::ty::Constants {
        let (borderColor, borderWidth) = match &self.figure.border {
            Some(border) => (border.color.as_ref().clone(), border.width),
            None => ([0.; 4], 0),
        };

        let cameraPos = match camera_2d {
            Some(camera_2d) => camera_2d.position.as_ref().clone(),
            None => [0.; 2],
        };

        figure_fs::ty::Constants {
            resolution: device_container.resolution(),
            position: self.figure.position.as_ref().clone(),
            borderColor,
            size: self.figure.size.as_ref().clone(),
            borderWidth,
            cameraPos,
        }
    }

    fn create_buffers(device_container: &mut DeviceContainer) -> BufferContainer2DUv {
        let vertices = vec![
            Vertex2DUv {
                position: [-0.5, -0.5],
                uvCoord: [0., 0.],
            },
            Vertex2DUv {
                position: [0.5, -0.5],
                uvCoord: [1., 0.],
            },
            Vertex2DUv {
                position: [-0.5, 0.5],
                uvCoord: [0., 1.],
            },
            Vertex2DUv {
                position: [0.5, 0.5],
                uvCoord: [1., 1.],
            },
        ];

        let indices = vec![0, 1, 2, 2, 1, 3];

        create_buffers_2d_uv(device_container, vertices, indices)
    }
}
