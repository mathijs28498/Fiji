use nalgebra_glm::{Vec3, Vec4};

use crate::public::objects::DEFAULT_COLOR;

use super::{DEFAULT_POSITION_3D, DEFAULT_ROTATION_3D, DEFAULT_SIZE_3D};

#[derive(Clone, Debug)]
pub struct Block {
    pub color: Vec4,
    pub position: Vec3,
    pub size: Vec3,
    pub rotation: Vec3,
}

impl Block {
    pub fn new_default() -> Self {
        Self {
            color: DEFAULT_COLOR,
            position: DEFAULT_POSITION_3D,
            size: DEFAULT_SIZE_3D,
            rotation: DEFAULT_ROTATION_3D,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn with_size(mut self, size: Vec3) -> Self {
        self.size = size;
        self
    }

    pub fn with_rotation(mut self, rotation: Vec3) -> Self {
        self.rotation = rotation;
        self
    }
}
