use nalgebra::Point3;
use nalgebra_glm::{Vec3, Mat4};

pub struct Camera3D {
    pub position: Vec3,
    pub dir: Vec3,
}

impl Camera3D {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0., 0., 3.),
            dir: Vec3::new(0., 0., -1.),
        }
    }

    pub(crate) fn get_view_matrix(&self) -> Mat4{
        let target = self.position + self.dir;
        Mat4::look_at_rh(&Point3::new(self.position.x, self.position.y, self.position.z), &Point3::new(target.x, target.y, target.z), &Vec3::new(0., 1., 0.))
    }
}