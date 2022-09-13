use draw_objects::{background::Background, circle::Circle, polygon::Polygon, rect::Rect, Border};

use input::{
    input_enums::{KeyCode, MouseButton},
    Input,
};
use nalgebra_glm::{Vec2, Vec3, Vec4};

mod draw_objects;
mod input;
mod rendering;

use crate::rendering::context::*;

fn main() {
    let mut context = Context::new(1280, 720);

    let mut pos = Vec2::new(100., 100.);
    let mut border_width = 0;

    let mut polygon_points = Vec::new();
    let mut draw_polygon = false;

    context.event_loop().run(move |input: &Input| {
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

        if input.key_pressed(&KeyCode::Z) {
            draw_polygon = !draw_polygon;
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

        context.draw_background(Background::new(Vec3::new(0., 0., 0.)));

        context.draw_rect(Rect::new(
            Vec4::new(0., 0.5, 0.5, 1.),
            input.mouse_position().clone(),
            Vec2::new(50., 50.),
            None,
        ));

        context.draw_circle(Circle::new(
            Vec4::new(1., 0., 1., 0.2),
            Vec2::new(620., 340.),
            300.,
            Some(Border::new(Vec4::new(1., 1., 1., 1.), border_width)),
        ));

        context.draw_rect(Rect::new(
            Vec4::new(1., 0.5, 1., 1.),
            Vec2::new(430., 325.),
            Vec2::new(20., 50.),
            None,
        ));

        context.draw_rect(Rect::new(
            Vec4::new(1., 1., 0., 1.),
            pos.clone(),
            Vec2::new(20., 50.),
            Some(Border::new(Vec4::new(1., 1., 1., 1.), border_width)),
        ));

        if draw_polygon && polygon_points.len() > 0 {
            context.draw_polygon(Polygon::new(
                Vec4::new(0.9, 0.3, 0.5, 1.),
                polygon_points.clone(),
                None,
            ));
        }

        for p in &polygon_points {
            context.draw_circle(Circle::new(Vec4::new(1., 1., 1., 1.), p.clone(), 10., None))
        }

        context.render();
    });
}
