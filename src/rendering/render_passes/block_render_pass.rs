use std::{f32::consts::FRAC_2_PI, sync::Arc};

use vulkano::{
    buffer::{DeviceLocalBuffer, TypedBufferAccess},
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
    render_pass::{Framebuffer, FramebufferCreateInfo, Subpass},
    sync::GpuFuture,
};

use nalgebra_glm::{Mat4, Vec3, Vec4};

use crate::rendering::{data_types::Vertex3D, device_container::DeviceContainer};

mod vs {
    vulkano_shaders::shader!(
        ty: "vertex",
        path: "src/shaders/block_render_pass.vert",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    );
}
mod fs {
    vulkano_shaders::shader!(
        ty: "fragment",
        path: "src/shaders/block_render_pass.frag",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    );
}

pub(crate) struct BlockRenderPass {
    pipeline: Arc<GraphicsPipeline>,
    viewport: Viewport,
    framebuffers: Vec<Arc<Framebuffer>>,
}

impl BlockRenderPass {
    pub(crate) fn new(device_container: &DeviceContainer) -> BlockRenderPass {
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

        let viewport = Viewport {
            origin: [0., 0.],
            dimensions: device_container.resolution_f32(),
            depth_range: 0.0..1.0,
        };

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

        Self {
            pipeline,
            viewport,
            framebuffers,
        }
    }

    pub(crate) fn draw(
        &mut self,
        device_container: &mut DeviceContainer,
        vertex_buffer: Arc<DeviceLocalBuffer<[Vertex3D]>>,
        index_buffer: Arc<DeviceLocalBuffer<[u32]>>,
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
                    clear_values: vec![None, None],
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

    pub(crate) fn create_push_constants(
        color: Vec4,
        position: Vec3,
        size: &Vec3,
        rotation: Vec3,
        view: Mat4,
        camera_pos: Vec3,
    ) -> fs::ty::Constants {
        let position = Vec3::new(position.x, -position.y, position.z);
        let world = Mat4::new_translation(&position)
            * Mat4::new_rotation(rotation)
            * Mat4::new_nonuniform_scaling(size);

        fs::ty::Constants {
            color: color.as_ref().clone(),
            world: world.as_ref().clone(),
            view: view.as_ref().clone(),
            proj: Mat4::new_perspective(1280. / 720., FRAC_2_PI, 0.0001, 1000.).as_ref().clone(),
            cameraPos: camera_pos.as_ref().clone(),
            resolution: [0, 0],
            _dummy0: [0; 4],
        }
    }
}
