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
use nalgebra_glm::{Vec2, Vec3, Vec4};

pub fn main() {
    let mut context = Context::new(1280, 720);
    context.background(Background::new_with_color(Vec3::new(0.3, 0.1, 0.1)));

    let r = Rect::new_default()
        .with_color(Vec4::new(0.7, 0.3, 0.7, 1.))
        .with_position(Vec2::new(400., 400.))
        .with_size(Vec2::new(100., 200.));

    let r_2 = r
        .clone()
        .with_color(Vec4::new(0., 1., 0., 1.))
        .with_position(r.position + Vec2::new(200., 0.));

    context.event_loop().run(move |input, event_handler| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        context.camera_2d.position.x += 0.1;

        context.ui_rect(r_2.clone());
        context.rect(r.clone());

        context.render();
    });
}
