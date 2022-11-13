use std::sync::Arc;
use vulkano::{
    buffer::TypedBufferAccess,
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents,
    },
    image::view::ImageView,
    pipeline::{
        graphics::{
            color_blend::ColorBlendState,
            input_assembly::InputAssemblyState,
            vertex_input::BuffersDefinition,
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline, Pipeline,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, Subpass},
    sync::GpuFuture,
};

use crate::rendering::{
    render_containers::device_container::DeviceContainer,
    render_objects::shared::{BufferContainer2D, Vertex2D},
};

pub(crate) mod poly_vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/shaders_2d/poly_pipeline.vert",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    }
}

pub(crate) mod poly_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/shaders_2d/poly_pipeline.frag",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    }
}

pub(crate) struct PolyPipeline {
    pipeline: Arc<GraphicsPipeline>,
    framebuffers: Vec<Arc<Framebuffer>>,
}

impl PolyPipeline {
    pub(crate) fn new(device_container: &DeviceContainer) -> Self {
        let vs = poly_vs::load(device_container.device().clone()).unwrap();
        let fs = poly_fs::load(device_container.device().clone()).unwrap();

        let render_pass = vulkano::single_pass_renderpass!(
            device_container.device().clone(),
            attachments: {
                color: {
                    load: Load,
                    store: Store,
                    format: device_container.image_format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap();

        let pipeline = GraphicsPipeline::start()
            .color_blend_state(ColorBlendState::blend_alpha(ColorBlendState::new(1)))
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .input_assembly_state(InputAssemblyState::new())
            .vertex_input_state(BuffersDefinition::new().vertex::<Vertex2D>())
            .vertex_shader(vs.entry_point("main").unwrap(), ())
            .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([
                Viewport {
                    origin: [0.0, 0.0],
                    dimensions: device_container.resolution_f32(),
                    depth_range: 0.0..1.0,
                },
            ]))
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .build(device_container.device().clone())
            .unwrap();

        let framebuffers = device_container
            .images()
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>();

        Self {
            pipeline,
            framebuffers,
        }
    }

    pub(crate) fn draw(
        &mut self,
        device_container: &mut DeviceContainer,
        buffers: &BufferContainer2D,
        mut push_constants: poly_fs::ty::Constants,
    ) {
        let mut builder = AutoCommandBufferBuilder::primary(
            device_container.command_buffer_allocator(),
            device_container.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![None],
                    ..RenderPassBeginInfo::framebuffer(
                        self.framebuffers[device_container.image_num()].clone(),
                    )
                },
                SubpassContents::Inline,
            )
            .unwrap()
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_vertex_buffers(0, buffers.vertex_buffer.clone())
            .bind_index_buffer(buffers.index_buffer.clone())
            .push_constants(self.pipeline.layout().clone(), 0, push_constants)
            .draw_indexed(buffers.index_buffer.len() as u32, 1, 0, 0, 0)
            .unwrap()
            .end_render_pass()
            .unwrap();

        device_container.execute_command_buffer(builder.build().unwrap());
    }
}
