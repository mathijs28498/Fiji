use draw_objects::{background::Background, square::Square, circle::Circle};

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use nalgebra_glm as glm;

mod draw_objects;
mod rendering;

use crate::rendering::common::*;

// TODO:
// [ ] - Add mouse input (look at bevy_vulkano)
// [ ] - Add keyboard input (look at bevy_vulkano)
// [ ] - Add initial width/height (look at bevy_vulkano)
// [ ] - Allow resizing (look at bevy_vulkano)
// [ ] - Make project a cargo crate
// [ ] - Add Polygon draw object

fn main() {
    let mut context = Context::new();

    context.event_loop().run(move |event, _, control_flow: &mut ControlFlow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::RedrawEventsCleared => {
            context.draw_background(Background::new(glm::Vec3::new(0., 0., 0.)));
            context.draw_square(Square::new(
                glm::Vec4::new(1., 1., 0., 1.),
                glm::Vec2::new(410., 325.),
                glm::Vec2::new(20., 50.),
            ));
            context.draw_square(Square::new(
                glm::Vec4::new(0., 1., 1., 1.),
                glm::Vec2::new(400., 300.),
                glm::Vec2::new(20., 50.),
            ));
            context.draw_circle(Circle::new(glm::Vec4::new(1., 0., 1., 0.2), glm::Vec2::new(500., 300.), 300.));
            context.render();
        }
        _ => (),
    });
}
