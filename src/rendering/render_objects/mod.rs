use self::{
    ro_2d::{
        circle_ro::CircleRenderObject, line_ro::LineRenderObject, polygon_ro::PolygonRenderObject,
        rect_ro::RectRenderObject, text_ro::TextRenderObject, figure_ro::FigureRenderObject,
    },
    ro_3d::block_ro::BlockRenderObject,
};

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
    TextObject(TextRenderObject),
    FigureObject(FigureRenderObject),
}
