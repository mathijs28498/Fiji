use std::f32::consts::FRAC_2_PI;

use nalgebra::Point3;
use nalgebra_glm::{Mat4, Vec3};

#[derive(Debug)]
pub struct Camera3D {
    pub position: Vec3,
    pub dir: Vec3,
    near: f32,
    far: f32,
    fov: f32,
}

impl Camera3D {
    pub fn new_default() -> Self {
        Self {
            position: Vec3::new(0., 0., 3.),
            dir: Vec3::new(0., 0., -1.),
            near: 0.01,
            far: 1000.,
            fov: FRAC_2_PI,
        }
    }

    pub(crate) fn get_view_matrix(&self) -> Mat4 {
        let target = self.position + self.dir;
        Mat4::look_at_rh(
            &Point3::new(self.position.x, self.position.y, self.position.z),
            &Point3::new(target.x, target.y, target.z),
            &Vec3::new(0., 1., 0.),
        )
    }

    pub(crate) fn get_proj_matrix(&self, resolution: [f32; 2]) -> Mat4 {
        Mat4::new_perspective(resolution[0] / resolution[1], self.fov, self.near, self.far)
    }
}
