use self::ro_2d::circle_ro::CircleRenderObject;

pub(super) mod background_ro;
pub(super) mod ro_2d;
pub(super) mod shared;

#[derive(Clone)]
pub(super) enum RenderObject2D {
    CircleObject(CircleRenderObject),
}
