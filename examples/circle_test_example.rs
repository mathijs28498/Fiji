use fiji::{Background, Circle, Color, Context, KeyCode};
use nalgebra_glm::Vec3;

fn main() {
    let mut context = Context::new(1280, 720);
    context.background(Background::new_with_color(Vec3::new(0., 0., 0.)));

    let circle = Circle {
        position: [500., 300.].into(),
        color: Color::Rgb(0., 1., 1.),
        radius: 50.,
        ..Default::default()
    };

    context.run(move |input, event_handler, context| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        context.draw_2d(circle.clone());
    });
}
