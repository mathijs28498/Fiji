use fiji::{Background, Circle, Context, KeyCode};
use nalgebra_glm::{Vec3, Vec4};

fn main() {
    let mut context = Context::new(1280, 720);
    context.background(Background::new_with_color(Vec3::new(0., 0., 0.)));

    context.run(move |input, event_handler, context| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        context.circle(
            Circle::new_default()
                .with_position([300., 300.].into())
                .with_radius(50.)
                .with_color(Vec4::new(0., 1., 1., 1.)),
        );
    });
}
