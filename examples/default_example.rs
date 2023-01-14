use fiji::{Background, Block, Border, Circle, Context, KeyCode, MouseButton, Polygon, Rect};
use nalgebra_glm::{Vec3, Vec4};

pub fn main() {
    let mut context = Context::new(1280, 720);

    context.run(move |input, event_handler, context| {
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

        context.render(event_handler);
    })
}
