use fiji::{Background, Circle, Context, KeyCode, Rect, Text};
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
        .with_color(Vec4::new(0., 1., 0., 0.3))
        .with_position(r.position + Vec2::new(200., 0.));

    context.event_loop().run(move |input, event_handler| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        context.camera_2d.position += Vec2::new(50. * context.dt() as f32, 0.);

        context.rect(r.clone());
        context.text(
            Text::new_with_text(&format!(
                "fps: {:.3} ({:.3}ms)",
                context.fps(),
                context.dt_micros() as f64 / 1000.
            ))
            .with_position(Vec2::new(200., 100.))
            .with_color(Vec4::new(0.9, 0.7, 0.2, 1.)),
        );
        context.circle(
            Circle::new_default()
                .with_radius(50.)
                .with_position(Vec2::new(700., 400.)),
        );

        context.ui_rect(r_2.clone());
        context.ui_circle(
            Circle::new_default()
                .with_radius(50.)
                .with_position(Vec2::new(600., 200.)),
        );

        let text = Text::new_with_text(&format!(
            "fps: {:.3} ({:.3}ms)",
            context.fps(),
            context.dt_micros() as f64 / 1000.
        ));

        for i in 0..0 {
            context.ui_text(
                text.clone()
                    .with_position(Vec2::new(50., 50. + i as f32 * 20.)),
            );
        }

        context.render(event_handler);
    });
}
