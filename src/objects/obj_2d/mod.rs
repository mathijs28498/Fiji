use nalgebra_glm::Vec2;

pub mod circle;
pub mod line;
pub mod polygon;
pub mod rect;

const DEFAULT_POSITION_2D: Vec2 = Vec2::new(50., 50.);
const DEFAULT_SIZE_2D: Vec2 = Vec2::new(20., 20.);
#[allow(unused)]
const DEFAULT_ROTATION_2D: Vec2 = Vec2::new(0., 0.);