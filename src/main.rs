use draw_objects::{background::Background, circle::Circle, square::Square};

use input::{Input, input_enums::{KeyCode, MouseButton}};
use nalgebra_glm as glm;

mod draw_objects;
mod rendering;
mod input;

use crate::rendering::common::*;

// TODO:
// [X] - Add mouse input (look at bevy_vulkano)
// [X] - Add keyboard input (look at bevy_vulkano)
// [ ] - Add initial width/height (look at bevy_vulkano)
// [ ] - Allow resizing (look at bevy_vulkano)
// [ ] - Make project a cargo crate
// [ ] - Add Polygon draw object
// [ ] - Rewrite code to use glam in stead of nalgebra_glm

fn main() {
    let mut context = Context::new();

    let mut pos = glm::Vec2::new(100., 100.);

    context.event_loop().run(move |input: &Input| {

        if input.key_held(&KeyCode::A) {
            pos += glm::Vec2::new(-1., 0.);
        }
        if input.key_held(&KeyCode::D) {
            pos += glm::Vec2::new(1., 0.);
        }
        if input.key_held(&KeyCode::W) {
            pos += glm::Vec2::new(0., -1.);
        }
        if input.key_held(&KeyCode::S) {
            pos += glm::Vec2::new(0., 1.);
        }

        if input.mouse_button_held(&MouseButton::Left) {
            context.draw_background(Background::new(glm::Vec3::new(1., 1., 1.)));
        } else {
            context.draw_background(Background::new(glm::Vec3::new(0., 0., 0.)));
        }


        context.draw_square(Square::new(
            glm::Vec4::new(0., 1., 1., 1.),
            input.mouse_position().clone(),
            glm::Vec2::new(20., 50.),
        ));

        context.draw_circle(Circle::new(
            glm::Vec4::new(1., 0., 1., 0.2),
            glm::Vec2::new(500., 300.),
            300.,
        ));

        context.draw_square(Square::new(
            glm::Vec4::new(1., 0.5, 1., 1.),
            glm::Vec2::new(430., 325.),
            glm::Vec2::new(20., 50.),
        ));

        context.draw_square(Square::new(
            glm::Vec4::new(1., 1., 0., 1.),
            pos.clone(),
            glm::Vec2::new(20., 50.),
        ));

        context.render();
    });
}
