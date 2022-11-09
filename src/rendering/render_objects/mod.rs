use crate::objects::{
    obj_2d::{circle::Circle, line::Line, polygon::Polygon, rect::Rect},
    obj_3d::block::Block,
};

use self::{ro_3d::block_ro::BlockRenderObject, ro_2d::{rect_ro::RectRenderObject, line_ro::LineRenderObject, polygon_ro::PolygonRenderObject, circle_ro::CircleRenderObject}};

pub(super) mod background_ro;
pub(super) mod ro_2d;
pub(super) mod ro_3d;
pub(super) mod shared;

#[derive(Clone)]
pub(super) enum RenderObject3D {
    BlockObject(BlockRenderObject),
}

#[derive(Clone)]
pub(super) enum RenderObject2D {
    RectObject(RectRenderObject),
    CircleObject(CircleRenderObject),
    LineObject(LineRenderObject),
    PolyObject(PolygonRenderObject),
}
