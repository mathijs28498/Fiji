use nalgebra_glm::{Vec2, Vec3, Vec4};

use fiji::{
    objects::{
        background::Background,
        obj_2d::{circle::Circle, polygon::Polygon, rect::Rect},
        Border,
    },
    input::input_enums::{KeyCode, MouseButton},
    rendering::context::Context,
};

fn main() {
    let mut v0 = Vec3::new(1., 4., 3.);
    let mut v1 = Vec3::new(3., 5., 1.);

    let v2 = v0 + v1;
    println!("{v0:?} - {v1:?} - {v2:?}");

    let mut context = Context::new(1280, 720);

    let mut pos = Vec2::new(100., 100.);
    let mut border_width = 0;

    let mut polygon_points = Vec::new();

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

        context.background(Background::new(Vec3::new(0., 0., 0.)));

        context.rect(Rect::new(
            Vec4::new(0., 0.5, 0.5, 1.),
            input.mouse_position().clone(),
            Vec2::new(50., 50.),
            None,
        ));

        context.circle(Circle::new(
            Vec4::new(1., 0., 1., 0.2),
            Vec2::new(620., 340.),
            300.,
            Some(Border::new(Vec4::new(1., 1., 1., 1.), border_width)),
        ));

        context.rect(Rect::new(
            Vec4::new(1., 0.5, 1., 1.),
            Vec2::new(430., 325.),
            Vec2::new(20., 50.),
            None,
        ));

        context.rect(Rect::new(
            Vec4::new(1., 1., 0., 1.),
            pos.clone(),
            Vec2::new(20., 50.),
            Some(Border::new(Vec4::new(1., 1., 1., 1.), border_width)),
        ));

        if polygon_points.len() >= 3 {
            context.polygon(Polygon::new(
                Vec4::new(0.9, 0.3, 0.5, 1.),
                polygon_points.clone(),
                None,
            ));
        }

        for p in &polygon_points {
            context.circle(Circle::new(Vec4::new(1., 1., 1., 1.), p.clone(), 10., None))
        }

        context.render();
    });
}
