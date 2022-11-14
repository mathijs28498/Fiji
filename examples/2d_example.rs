use nalgebra_glm::{Vec2, Vec3, Vec4};

use fiji::{Background, Border, Circle, Context, KeyCode, MouseButton, Polygon, Rect, Text};

fn main() {
    let mut context = Context::new(1280, 720);

    let mut pos = Vec2::new(100., 100.);
    let mut border_width = 0;

    let mut polygon_points = Vec::new();
    let mut timer = 0.;

    context.event_loop().run(move |input, fiji_event_handler| {
        if input.key_pressed(&KeyCode::Escape) {
            fiji_event_handler.exit();
        }

        if input.key_pressed(&KeyCode::Q) {
            if border_width > 0 {
                border_width -= 1;
            }
        }

        if input.key_pressed(&KeyCode::E) {
            border_width += 1;
        }

        if input.key_held(&KeyCode::A) {
            pos += Vec2::new(-1., 0.);
        }
        if input.key_held(&KeyCode::D) {
            pos += Vec2::new(1., 0.);
        }
        if input.key_held(&KeyCode::W) {
            pos += Vec2::new(0., -1.);
        }
        if input.key_held(&KeyCode::S) {
            pos += Vec2::new(0., 1.);
        }

        if input.mouse_button_pressed(&MouseButton::Left) {
            polygon_points.push(input.mouse_position().clone());
        }
        if input.key_pressed(&KeyCode::X) {
            if polygon_points.len() > 0 {
                polygon_points.remove(polygon_points.len() - 1);
            }
        }
        if input.key_pressed(&KeyCode::C) {
            polygon_points = Vec::new();
        }
        timer += 0.03;

        context.background(Background::new_with_color(Vec3::new(0., 0., 0.)));

        context.ui_text(Text::new_with_text(&format!("{:.2}", timer)));

        context.rect(
            Rect::new_default()
                .with_color(Vec4::new(0., 0.5, 0.5, 1.))
                .with_position(input.mouse_position().clone())
                .with_size(Vec2::new(50., 50.)),
        );

        context.circle(
            Circle::new_default()
                .with_color(Vec4::new(1., 0., 1., 0.2))
                .with_position(Vec2::new(620., 340.))
                .with_radius(300.)
                .with_border(Border::new(Vec4::new(1., 1., 1., 1.), border_width)),
        );

        context.rect(
            Rect::new_default()
                .with_color(Vec4::new(1., 0.5, 1., 1.))
                .with_position(Vec2::new(430., 325.))
                .with_size(Vec2::new(20., 50.)),
        );

        context.rect(
            Rect::new_default()
                .with_color(Vec4::new(1., 1., 0., 1.))
                .with_position(pos.clone())
                .with_size(Vec2::new(20., 50.))
                .with_border(Border::new(Vec4::new(1., 1., 1., 1.), border_width)),
        );

        if polygon_points.len() >= 3 {
            context.polygon(
                Polygon::new_with_points(polygon_points.clone())
                    .with_color(Vec4::new(0.9, 0.3, 0.5, 1.)),
            );
        }

        for p in &polygon_points {
            context.circle(
                Circle::new_default()
                    .with_position(p.clone())
                    .with_radius(10.),
            )
        }

        context.render(fiji_event_handler);
    });
}
