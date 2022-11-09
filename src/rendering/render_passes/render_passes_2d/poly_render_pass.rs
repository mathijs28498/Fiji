use std::sync::Arc;
use vulkano::{
    buffer::TypedBufferAccess,
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents,
    },
    image::{view::ImageView, ImageAccess},
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

use crate::{
    objects::Border,
    rendering::{
        render_containers::device_container::DeviceContainer,
        render_objects::shared::{BufferContainer2D, Vertex2D},
    },
};

use nalgebra_glm::{Vec2, Vec4};
mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/shaders_2d/poly_render_pass.vert",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/shaders_2d/poly_render_pass.frag",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    }
}

pub(crate) struct PolyRenderPass {
    pipeline: Arc<GraphicsPipeline>,
    viewport: Viewport,
    framebuffers: Vec<Arc<Framebuffer>>,
}

impl PolyRenderPass {
    pub(crate) fn new(device_container: &DeviceContainer) -> Self {
        let vs = vs::load(device_container.device().clone()).unwrap();
        let fs = fs::load(device_container.device().clone()).unwrap();

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
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .build(device_container.device().clone())
            .unwrap();

        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };

        let dimensions = device_container.images()[0].dimensions().width_height();
        viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

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
            viewport,
            framebuffers,
        }
    }

    pub(crate) fn draw(
        &mut self,
        device_container: &mut DeviceContainer,
        buffers: &BufferContainer2D,
        mut push_constants: fs::ty::Constants,
    ) {
        push_constants.resolution = device_container.resolution();

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
            .set_viewport(0, [self.viewport.clone()])
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_vertex_buffers(0, buffers.vertex_buffer.clone())
            .bind_index_buffer(buffers.index_buffer.clone())
            .push_constants(self.pipeline.layout().clone(), 0, push_constants)
            .draw_indexed(buffers.index_buffer.len() as u32, 1, 0, 0, 0)
            .unwrap()
            .end_render_pass()
            .unwrap();

        let command_buffer = builder.build().unwrap();

        device_container.previous_frame_end = Some(
            device_container
                .previous_frame_end
                .take()
                .unwrap()
                .then_execute(device_container.queue().clone(), command_buffer)
                .unwrap()
                .boxed(),
        );
    }

    pub(crate) fn create_push_constants(
        color: Vec4,
        position: Vec2,
        size: Vec2,
        border: Option<Border>,
    ) -> fs::ty::Constants {
        let (border_color, border_width) = match border {
            Some(border) => (border.color, border.width),
            None => (Vec4::new(0., 0., 0., 0.), 0),
        };
        fs::ty::Constants {
            resolution: [0, 0],
            position: position.as_ref().clone(),
            color: color.as_ref().clone(),
            borderColor: border_color.as_ref().clone(),
            size: size.as_ref().clone(),
            borderWidth: border_width,
        }
    }
}
