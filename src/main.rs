use draw_objects::{background::Background, square::Square};

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use nalgebra_glm as glm;

mod draw_objects;
mod rendering;

use crate::rendering::common::*;

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
                glm::Vec2::new(400., 300.),
                glm::Vec2::new(20., 50.),
                glm::Vec3::new(0., 1., 1.),
            ));
            context.render();
        }
        _ => (),
    });
}
