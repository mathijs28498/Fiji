use nalgebra_glm::{Vec2, Vec4};

use crate::{
    public::objects::{Border, DEFAULT_COLOR},
    rendering::{
        pipelines::pipelines_2d::circle_pipeline::{circle_fs, CIRCLE_PIPELINE},
        render_containers::{device_container::DeviceContainer, render_container::Drawable2D},
    },
    shared::{BufferContainer2D, create_buffers_2d, Vertex2D},
    Camera2D, Color,
};

use super::DEFAULT_POSITION_2D;

#[derive(Clone, Debug)]
pub struct Circle {
    pub color: Color,
    pub position: Vec2,
    pub radius: f32,
    pub border: Option<Border>,
    buffers: Option<BufferContainer2D>,
}

impl Circle {
    pub fn new_default() -> Self {
        Self {
            color: DEFAULT_COLOR,
            position: DEFAULT_POSITION_2D,
            radius: 10.,
            border: None,
            buffers: None,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    #[allow(non_snake_case)]
    fn create_push_constants(
        &self,
        device_container: &DeviceContainer,
        camera_2d: Option<&Camera2D>,
    ) -> circle_fs::ty::Constants {
        let (borderColor, borderWidth) = match &self.border {
            Some(border) => (border.color.clone().into(), border.width),
            None => ([0.; 4], 0),
        };

        let cameraPos = match camera_2d {
            Some(camera_2d) => camera_2d.position.as_ref().clone(),
            None => [0.; 2],
        };

        circle_fs::ty::Constants {
            resolution: device_container.resolution(),
            color: self.color.clone().into(),
            position: self.position.as_ref().clone(),
            borderColor,
            borderWidth,
            radius: self.radius,
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

impl Drawable2D for Circle {
    fn set_buffers(&mut self, device_container: &mut DeviceContainer) {
        if let None = self.buffers {
            static mut BUFFERS: Option<BufferContainer2D> = None;

            // Unsafe is used to change these static values.
            // This is definitely safe, even thought the compiler can't verify.
            unsafe {
                if let None = BUFFERS {
                    BUFFERS = Some(Self::create_buffers(device_container));
                }
                self.buffers = Some(BUFFERS.as_ref().unwrap().clone());
            };
        };
    }

    fn draw(&mut self, device_container: &mut DeviceContainer, camera_2d: Option<&Camera2D>) {
        let binding = CIRCLE_PIPELINE.get().unwrap().clone();
        let mut pipeline = binding.write().unwrap();
        pipeline.draw(
            device_container,
            self.buffers.as_ref().unwrap(),
            self.create_push_constants(device_container, camera_2d),
        );
    }
}
