#![allow(unused)]

use fiji::{Background, Circle, CircleBuilder, Color, Context, KeyCode};
use nalgebra_glm::Vec3;

fn main() {
    let mut context = Context::new(1280, 720);
    context.background(Background::new_with_color(Vec3::new(0., 0., 0.)));

    let circle = CircleBuilder::new()
        .color(Color::Rgb(1., 1., 0.))
        .radius(20.)
        .position([300., 300.].into())
        .build();

    context.run(move |input, event_handler, context| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        context.draw_2d(circle.clone());
    });
}
