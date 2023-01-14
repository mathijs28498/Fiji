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
    pipeline: Arc<GraphicsPipeline>,
}

impl BlockPipeline {
    pub(crate) fn new(device_container: &DeviceContainer) -> BlockPipeline {
        let vs = block_vs::load(device_container.device().clone()).unwrap();
        let fs = block_fs::load(device_container.device().clone()).unwrap();
        let pipeline = Self::create_pipeline(device_container, &vs, &fs);

        Self { vs, fs, pipeline }
    }

    fn create_pipeline(
        device_container: &DeviceContainer,
        vs: &Arc<ShaderModule>,
        fs: &Arc<ShaderModule>,
    ) -> Arc<GraphicsPipeline> {
        GraphicsPipeline::start()
            .color_blend_state(ColorBlendState::blend_alpha(ColorBlendState::new(1)))
            .input_assembly_state(InputAssemblyState::new())
            .rasterization_state(RasterizationState::new().cull_mode(CullMode::Back))
            .render_pass(Subpass::from(device_container.render_pass().clone(), 0).unwrap())
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
            .unwrap()
    }

    pub(crate) fn recreate_pipeline(&mut self, device_container: &DeviceContainer) {
        self.pipeline = Self::create_pipeline(device_container, &self.vs, &self.fs);
    }

    pub(crate) fn draw(
        &mut self,
        device_container: &mut DeviceContainer,
        buffers: &BufferContainer3D,
        mut push_constants: block_fs::ty::Constants,
    ) {
        push_constants.resolution = device_container.resolution();

        let mut builder = device_container.get_command_buffer_builder();

        builder
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_vertex_buffers(0, buffers.vertex_buffer.clone())
            .bind_index_buffer(buffers.index_buffer.clone())
            .push_constants(self.pipeline.layout().clone(), 0, push_constants)
            .draw_indexed(buffers.index_buffer.len() as u32, 1, 0, 0, 0)
            .unwrap();
    }
}
