use std::sync::Arc;

use vulkano::{
    buffer::TypedBufferAccess,
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents,
    },
    image::view::ImageView,
    pipeline::{
        self,
        graphics::{
            color_blend::ColorBlendState,
            depth_stencil::DepthStencilState,
            input_assembly::InputAssemblyState,
            rasterization::{CullMode, RasterizationState},
            vertex_input::BuffersDefinition,
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline, Pipeline,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    shader::ShaderModule,
    sync::GpuFuture,
};

use crate::rendering::{
    render_containers::device_container::DeviceContainer,
    render_objects::shared::{BufferContainer3D, Vertex3D},
};

pub(crate) mod block_vs {
    vulkano_shaders::shader!(
        ty: "vertex",
        path: "src/shaders/shaders_3d/block_pipeline.vert",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    );
}

pub(crate) mod block_fs {
    vulkano_shaders::shader!(
        ty: "fragment",
        path: "src/shaders/shaders_3d/block_pipeline.frag",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    );
}

pub(crate) struct BlockPipeline {
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    framebuffers: Vec<Arc<Framebuffer>>,
}

impl BlockPipeline {
    pub(crate) fn new(device_container: &DeviceContainer) -> BlockPipeline {
        let vs = block_vs::load(device_container.device().clone()).unwrap();
        let fs = block_fs::load(device_container.device().clone()).unwrap();

        let render_pass = vulkano::single_pass_renderpass!(
            device_container.device().clone(),
            attachments: {
                color: {
                    load: Load,
                    store: Store,
                    format: device_container.image_format(),
                    samples: 1,
                },
                depth: {
                    load: Load,
                    store: Store,
                    format: device_container.depth_image_format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth}
            }
        )
        .unwrap();

        let (pipeline, framebuffers) =
            Self::create_pipeline(device_container, &vs, &fs, &render_pass);

        Self {
            vs,
            fs,
            render_pass,
            pipeline,
            framebuffers,
        }
    }

    fn create_pipeline(
        device_container: &DeviceContainer,
        vs: &Arc<ShaderModule>,
        fs: &Arc<ShaderModule>,
        render_pass: &Arc<RenderPass>,
    ) -> (Arc<GraphicsPipeline>, Vec<Arc<Framebuffer>>) {
        let pipeline = GraphicsPipeline::start()
            .color_blend_state(ColorBlendState::blend_alpha(ColorBlendState::new(1)))
            .input_assembly_state(InputAssemblyState::new())
            .rasterization_state(RasterizationState::new().cull_mode(CullMode::Back))
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .vertex_input_state(BuffersDefinition::new().vertex::<Vertex3D>())
            .vertex_shader(vs.entry_point("main").unwrap(), ())
            .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([
                Viewport {
                    origin: [0.0, 0.0],
                    dimensions: device_container.resolution_f32(),
                    depth_range: 0.0..1.0,
                },
            ]))
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .depth_stencil_state(DepthStencilState::simple_depth_test())
            .build(device_container.device().clone())
            .unwrap();

        let depth_view = ImageView::new_default(device_container.depth_image().clone()).unwrap();

        let framebuffers = device_container
            .images()
            .iter()
            .map(|image| {
                let image_view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![image_view, depth_view.clone()],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>();

        (pipeline, framebuffers)
    }

    pub(crate) fn recreate_pipeline(&mut self, device_container: &DeviceContainer) {
        (self.pipeline, self.framebuffers) = Self::create_pipeline(device_container, &self.vs, &self.fs, &self.render_pass)
    }

    pub(crate) fn draw(
        &mut self,
        device_container: &mut DeviceContainer,
        buffers: &BufferContainer3D,
        mut push_constants: block_fs::ty::Constants,
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
                    clear_values: vec![None, None],
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
