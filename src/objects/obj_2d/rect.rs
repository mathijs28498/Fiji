use nalgebra_glm::{Vec2, Vec4};

use crate::{
    objects::{help_functions::create_buffers_2d, Border, DEFAULT_COLOR},
    rendering::{
        data_types::{BufferContainer2D, Vertex2D},
        device_container::DeviceContainer,
        render_passes::poly_render_pass::PolyRenderPass,
    },
};

use super::{DEFAULT_POSITION_2D, DEFAULT_SIZE_2D};

#[derive(Clone, Debug)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Vec4,
    pub border: Option<Border>,
}

impl Rect {
    pub fn new_default() -> Self {
        Self {
            color: DEFAULT_COLOR,
            position: DEFAULT_POSITION_2D,
            size: DEFAULT_SIZE_2D,
            border: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut PolyRenderPass,
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

        render_pass.draw(
            device_container,
            buffers,
            PolyRenderPass::create_push_constants(
                self.color.clone(),
                self.position.clone(),
                self.size.clone(),
                self.border.clone(),
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
