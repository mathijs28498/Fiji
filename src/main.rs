use bytemuck::{Pod, Zeroable};
use std::sync::Arc;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents,
    },
    device::Queue,
    format::Format,
    image::{view::ImageView, ImageAccess, SwapchainImage},
    impl_vertex,
    pipeline::{
        graphics::{
            input_assembly::InputAssemblyState,
            vertex_input::BuffersDefinition,
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    swapchain::{acquire_next_image, AcquireError, SwapchainCreateInfo, SwapchainCreationError},
    sync::{self, FlushError, GpuFuture},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod rendering;
use crate::rendering::{common::*, data_types::*, render_passes::*};


fn main() {
    let event_loop = EventLoop::new();
    let mut context = Context::new(&event_loop);

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

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::RedrawEventsCleared => {
            context.draw_vertex_buffer(&vertex_buffer);
            context.draw_vertex_buffer(&vertex_buffer_2);
            context.render();
        }
        _ => (),
    });
}
