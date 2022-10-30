use std::{sync::Arc, f32::consts::{PI, FRAC_2_PI, FRAC_PI_2}};

use nalgebra::Point3;
use vulkano::{
    buffer::{ImmutableBuffer, TypedBufferAccess},
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

use nalgebra_glm::{Vec4, Mat4, Vec3};

use crate::rendering::{data_types::Vertex3D, device_container::DeviceContainer};

#[derive(Debug)]
pub(crate) struct BlockPushConstants {
    _color: Vec4,
    _model: Mat4,
    _view: Mat4,
    _proj: Mat4,
    _resolution: [u32; 2],
}

impl BlockPushConstants {
    pub(crate) fn new(color: Vec4, position: Vec4, size: &Vec3, view: Mat4) -> BlockPushConstants {
        // TODO: Get proper aspect (not hardcoded)
        Self {
            _color: color,
            _model: Mat4::new_nonuniform_scaling(size) * Mat4::new_rotation(Vec3::new(1., 0.5, -2.)),
            _view: view,
            _proj: Mat4::new_perspective(1280./720., FRAC_2_PI, 0.0001, 1000.),
            _resolution: [0, 0],
        }
    }
}

pub(crate) struct BlockRenderPass {
    pipeline: Arc<GraphicsPipeline>,
    viewport: Viewport,
    framebuffers: Vec<Arc<Framebuffer>>,
}

impl BlockRenderPass {
    pub(crate) fn new(device_container: &DeviceContainer) -> BlockRenderPass {
        mod vs {
            vulkano_shaders::shader!(
                ty: "vertex",
                path: "src/shaders/block_render_pass.vert"
            );
        }
        mod fs {
            vulkano_shaders::shader!(
                ty: "fragment",
                path: "src/shaders/block_render_pass.frag"
            );
        }

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
            .input_assembly_state(InputAssemblyState::new())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .vertex_input_state(BuffersDefinition::new().vertex::<Vertex3D>())
            .vertex_shader(vs.entry_point("main").unwrap(), ())
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .build(device_container.device().clone())
            .unwrap();

        let viewport = Viewport {
            origin: [0., 0.],
            dimensions: device_container.resolution_f32(),
            depth_range: 0.0..1.0,
        };

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
        vertex_buffer: Arc<ImmutableBuffer<[Vertex3D]>>,
        index_buffer: Arc<ImmutableBuffer<[u32]>>,
        mut push_constants: BlockPushConstants,
    ) {
        push_constants._resolution = device_container.resolution();

        let mut builder = AutoCommandBufferBuilder::primary(
            device_container.device().clone(),
            device_container.queue().family(),
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
            .bind_vertex_buffers(0, vertex_buffer.clone())
            .bind_index_buffer(index_buffer.clone())
            .push_constants(self.pipeline.layout().clone(), 0, push_constants)
            .draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
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
}
