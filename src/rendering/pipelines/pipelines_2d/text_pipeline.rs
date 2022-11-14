use std::{collections::HashMap, sync::Arc};

use fontdue::{Font, FontSettings, Metrics};
use vulkano::{
    buffer::TypedBufferAccess,
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract,
        RenderPassBeginInfo, SubpassContents,
    },
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    format::Format,
    image::{view::ImageView, ImageDimensions, ImmutableImage},
    pipeline::{
        graphics::{
            color_blend::ColorBlendState,
            input_assembly::InputAssemblyState,
            vertex_input::BuffersDefinition,
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline, Pipeline, PipelineBindPoint,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo},
    shader::ShaderModule,
    sync::GpuFuture,
};

use crate::{
    public::objects::obj_2d::text::TextFont,
    rendering::{
        render_containers::device_container::DeviceContainer,
        render_objects::shared::{create_buffers_text, BufferContainerText, VertexText},
    },
};

pub(crate) mod text_vs {
    vulkano_shaders::shader!(
        ty: "vertex",
        path: "src/shaders/shaders_2d/text_pipeline.vert",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    );
}

pub(crate) mod text_fs {
    vulkano_shaders::shader!(
        ty: "fragment",
        path: "src/shaders/shaders_2d/text_pipeline.frag",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    );
}

pub(crate) struct TextPipeline {
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    framebuffers: Vec<Arc<Framebuffer>>,
    font_sets: HashMap<char, (Option<Arc<PersistentDescriptorSet>>, Metrics)>,
    font_image_sampler: Arc<Sampler>,
    comic_sans_font: Font,
}

impl TextPipeline {
    pub(crate) fn new(device_container: &DeviceContainer) -> Self {
        let vs = text_vs::load(device_container.device().clone()).unwrap();
        let fs = text_fs::load(device_container.device().clone()).unwrap();

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

        let (pipeline, framebuffers) =
            Self::create_pipeline(device_container, &vs, &fs, &render_pass);

        let font_image_sampler = Sampler::new(
            device_container.device().clone(),
            SamplerCreateInfo {
                mag_filter: Filter::Nearest,
                min_filter: Filter::Nearest,
                address_mode: [SamplerAddressMode::Repeat; 3],
                ..Default::default()
            },
        )
        .unwrap();

        let comic_sans_font = Font::from_bytes(
            include_bytes!("C:/Users/mathi/OneDrive/Documents/Fonts/comic-sans-ms/comicz.ttf")
                as &[u8],
            FontSettings::default(),
        )
        .unwrap();

        Self {
            vs,
            fs,
            render_pass,
            pipeline,
            framebuffers,
            font_sets: HashMap::new(),
            font_image_sampler,
            comic_sans_font,
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
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .vertex_input_state(BuffersDefinition::new().vertex::<VertexText>())
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

        (pipeline, framebuffers)
    }

    pub(crate) fn recreate_pipeline(&mut self, device_container: &DeviceContainer) {
        (self.pipeline, self.framebuffers) =
            Self::create_pipeline(device_container, &self.vs, &self.fs, &self.render_pass);
    }

    // TODO: Only wait for builder once in stead for each char
    pub(crate) fn get_or_create_set(
        &mut self,
        device_container: &DeviceContainer,
        c: char,
        font: &TextFont,
    ) -> (Option<Arc<PersistentDescriptorSet>>, Metrics) {
        if let Some(font_set) = self.font_sets.get(&c) {
            return font_set.clone();
        }

        let (metrics, bitmap);
        match font {
            TextFont::ComicSans => {
                (metrics, bitmap) = self.comic_sans_font.rasterize(c, 40.);
            }
        }

        if metrics.width == 0 {
            self.font_sets.insert(c, (None, metrics.clone()));
            return (None, metrics);
        }

        let mut builder = AutoCommandBufferBuilder::primary(
            device_container.command_buffer_allocator(),
            device_container.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let char_image_view = ImageView::new_default(
            ImmutableImage::from_iter(
                device_container.memory_allocator(),
                bitmap,
                ImageDimensions::Dim2d {
                    width: metrics.width as u32,
                    height: metrics.height as u32,
                    array_layers: 1,
                },
                1.into(),
                Format::R8_UINT,
                &mut builder,
            )
            .unwrap(),
        )
        .unwrap();

        let set = PersistentDescriptorSet::new(
            device_container.descriptor_set_allocator(),
            self.pipeline.layout().set_layouts().get(0).unwrap().clone(),
            [WriteDescriptorSet::image_view_sampler(
                0,
                char_image_view,
                self.font_image_sampler.clone(),
            )],
        )
        .unwrap();

        builder
            .build()
            .unwrap()
            .execute(device_container.queue().clone())
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None /* timeout */)
            .unwrap();

        (Some(set), metrics)
    }

    pub(crate) fn draw(
        &mut self,
        device_container: &mut DeviceContainer,
        push_constants: text_fs::ty::Constants,
        mut sets: Vec<(Option<Arc<PersistentDescriptorSet>>, Metrics)>,
    ) {
        let mut builder = AutoCommandBufferBuilder::primary(
            device_container.command_buffer_allocator(),
            device_container.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let mut max_height = 0.;
        for (set, metrics) in sets.iter() {
            if metrics.height as f32 > max_height {
                max_height = metrics.height as f32;
            }
        }

        let mut offset = 0.;
        for (set_option, metrics) in sets {
            if let None = set_option {
                offset += metrics.advance_width;
                continue;
            }

            let set = set_option.unwrap();

            let buffers = create_buffers(
                device_container,
                metrics,
                max_height - metrics.height as f32,
                offset,
            );
            offset += metrics.advance_width;

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
                .push_constants(self.pipeline.layout().clone(), 0, push_constants)
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    self.pipeline.layout().clone(),
                    0,
                    set,
                )
                .bind_vertex_buffers(0, buffers.vertex_buffer.clone())
                .bind_index_buffer(buffers.index_buffer.clone())
                .draw_indexed(buffers.index_buffer.len() as u32, 1, 0, 0, 0)
                .unwrap()
                .end_render_pass()
                .unwrap();
        }

        device_container.execute_command_buffer(builder.build().unwrap());
    }
}

fn create_buffers(
    device_container: &mut DeviceContainer,
    metrics: Metrics,
    y_min: f32,
    x_offset: f32,
) -> BufferContainerText {
    let x_min = x_offset;
    let vertices = vec![
        VertexText {
            position: [x_min, y_min],
            uvCoord: [0., 0.],
        },
        VertexText {
            position: [x_min + metrics.width as f32, y_min],
            uvCoord: [1., 0.],
        },
        VertexText {
            position: [x_min, y_min + metrics.height as f32],
            uvCoord: [0., 1.],
        },
        VertexText {
            position: [x_min + metrics.width as f32, y_min + metrics.height as f32],
            uvCoord: [1., 1.],
        },
    ];

    let indices = vec![0, 1, 2, 2, 1, 3];

    create_buffers_text(device_container, vertices, indices)
}
