use fiji::{
    input::input_enums::{KeyCode, MouseButton},
    objects::{
        background::Background,
        obj_2d::{circle::Circle, polygon::Polygon, rect::Rect},
        obj_3d::block::Block,
        Border,
    },
    rendering::context::Context,
};
use nalgebra_glm::{Vec3, Vec4};

pub fn main() {
    let mut context = Context::new(1280, 720);

    context.event_loop().run(move |input, event_handler| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        context.block(
            Block::new_default()
                .with_color(Vec4::new(0., 1., 0., 1.))
                .with_rotation(Vec3::new(1., -1., 0.)),
        );
        context.rect(Rect::new_default().with_color(Vec4::new(1., 0., 0., 1.)));
        context.circle(Circle::new_default());

        context.render();
    })
}
