use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use nalgebra_glm as glm;

mod draw_objects;
mod rendering;

use crate::draw_objects::*;
use crate::rendering::{common::*, data_types::*};

fn main() {
    let mut context = Context::new();

    let vertices = [
        Vertex {
            position: [-0.5, -0.25],
        },
        Vertex {
            position: [0.0, 0.5],
        },
        Vertex {
            position: [0.25, -0.1],
        },
    ];
    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        context.device().clone(),
        BufferUsage::all(),
        false,
        vertices,
    )
    .unwrap();

    let vertices = [
        Vertex {
            position: [-0., -0.25],
        },
        Vertex {
            position: [0.2, 0.5],
        },
        Vertex {
            position: [0.25, -0.1],
        },
    ];
    let vertex_buffer_2 = CpuAccessibleBuffer::from_iter(
        context.device().clone(),
        BufferUsage::all(),
        false,
        vertices,
    )
    .unwrap();

    context
        .event_loop
        .take()
        .unwrap()
        .run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawEventsCleared => {
                // context.draw_vertex_buffer(&vertex_buffer);
                // context.draw_vertex_buffer(&vertex_buffer_2);
                context.draw(Box::new(Square::new(
                    glm::Vec2::new(0., 0.),
                    glm::Vec2::new(0., 0.),
                    glm::Vec3::new(0., 0., 0.),
                )));
                context.render();
            }
            _ => (),
        });
}
