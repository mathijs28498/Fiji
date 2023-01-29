use std::{rc::Rc, sync::Arc};

use vulkano::{
    command_buffer::{
        allocator::{StandardCommandBufferAlloc, StandardCommandBufferAllocator},
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer,
        RenderPassBeginInfo, SubpassContents,
    },
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue,
        QueueCreateInfo,
    },
    format::Format,
    image::{view::ImageView, AttachmentImage, ImageAccess, ImageUsage, SwapchainImage},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{FreeListAllocator, GenericMemoryAllocator, StandardMemoryAllocator},
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainCreationError,
        SwapchainPresentInfo,
    },
    sync,
    sync::GpuFuture,
    VulkanLibrary,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::Background;


pub struct DeviceContainer {
    surface: Arc<Surface>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    depth_image: Arc<AttachmentImage>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,

    previous_frame_end: Option<Box<dyn GpuFuture>>,
    image_num: usize,

    command_buffer_builder: Option<
        AutoCommandBufferBuilder<
            PrimaryAutoCommandBuffer<StandardCommandBufferAlloc>,
            StandardCommandBufferAllocator,
        >,
    >,

    memory_allocator: Rc<GenericMemoryAllocator<Arc<FreeListAllocator>>>,
    command_buffer_allocator: Rc<StandardCommandBufferAllocator>,
    descriptor_set_allocator: Rc<StandardDescriptorSetAllocator>,
}

impl DeviceContainer {
    pub(crate) fn new(event_loop: &EventLoop<()>, width: u32, height: u32) -> Self {
        let library = VulkanLibrary::new().unwrap();
        let required_extensions = vulkano_win::required_extensions(&library);

        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                enumerate_portability: true,
                ..Default::default()
            },
        )
        .unwrap();

        let surface = {
            let surface = WindowBuilder::new()
                .build_vk_surface(&event_loop, instance.clone())
                .unwrap();
            let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
            window.set_inner_size(PhysicalSize::new(width, height));

            surface
        };

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.graphics
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .expect("No suitable physical device found");

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .unwrap();

        let memory_allocator = Rc::new(StandardMemoryAllocator::new_default(device.clone()));
        let command_buffer_allocator = Rc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));
        let descriptor_set_allocator = Rc::new(StandardDescriptorSetAllocator::new(device.clone()));

        let queue = queues.next().unwrap();
        let (swapchain, images) = {
            let surface_capabilities = device
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .unwrap();

            let image_format = Some(
                device
                    .physical_device()
                    .surface_formats(&surface, Default::default())
                    .unwrap()[0]
                    .0,
            );

            let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();

            Swapchain::new(
                device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count,
                    image_format,
                    image_extent: window.inner_size().into(),
                    image_usage: ImageUsage {
                        transfer_dst: true,
                        color_attachment: true,
                        ..ImageUsage::empty()
                    },
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .iter()
                        .next()
                        .unwrap(),

                    ..Default::default()
                },
            )
            .unwrap()
        };

        let depth_image = AttachmentImage::with_usage(
            memory_allocator.as_ref(),
            images[0].dimensions().width_height(),
            Format::D32_SFLOAT,
            ImageUsage {
                transfer_dst: true,
                ..ImageUsage::empty()
            },
        )
        .unwrap();

        // let render_pass = vulkano::single_pass_renderpass!(
        //     device.clone(),
        //     attachments: {
        //         color: {
        //             load: Clear,
        //             store: Store,
        //             format: swapchain.image_format(),
        //             samples: 1,
        //         }
        //     },
        //     pass: {
        //         color: [color],
        //         depth_stencil: {}
        //     }
        // )
        // .unwrap();
        let render_pass = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Load,
                    store: Store,
                    format: swapchain.image_format(),
                    samples: 1,
                },
                depth: {
                    load: Load,
                    store: Store,
                    format: depth_image.format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth}
            }
        )
        .unwrap();

        let depth_view = ImageView::new_default(depth_image.clone()).unwrap();
        let framebuffers = images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view, depth_view.clone()],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>();

        let previous_frame_end = Some(sync::now(queue.device().clone()).boxed());

        Self {
            surface,
            queue,
            swapchain,
            images,
            depth_image,
            render_pass,
            framebuffers,
            previous_frame_end,
            command_buffer_builder: None,
            image_num: 0,
            memory_allocator,
            command_buffer_allocator,
            descriptor_set_allocator,
        }
    }

    // TODO: Return error type in stead of bool
    pub(super) fn recreate_swapchain_images(&mut self) -> bool {
        let dimensions = self.dimensions();
        if dimensions.width == 0 || dimensions.height == 0 {
            return false;
        }
        (self.swapchain, self.images) = match self.swapchain.recreate(SwapchainCreateInfo {
            image_extent: dimensions.into(),
            ..self.swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return false,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };

        self.render_pass = vulkano::single_pass_renderpass!(
            self.device().clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: self.image_format(),
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: Store,
                    format: self.depth_image_format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth}
            }
        )
        .unwrap();

        let depth_view = ImageView::new_default(self.depth_image.clone()).unwrap();
        self.framebuffers = self
            .images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    self.render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view, depth_view.clone()],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>();

        self.depth_image = AttachmentImage::with_usage(
            self.memory_allocator().as_ref(),
            self.resolution(),
            Format::D32_SFLOAT,
            ImageUsage {
                transfer_dst: true,
                ..ImageUsage::empty()
            },
        )
        .unwrap();

        return true;
    }

    pub(super) fn begin_draw(&mut self, background: &Background) {
        self.execute_command_buffer();
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();
        let (image_num, _, acquire_future) =
            acquire_next_image(self.swapchain().clone(), None).unwrap();

        self.previous_frame_end = Some(
            self.previous_frame_end
                .take()
                .unwrap()
                .join(acquire_future)
                .boxed(),
        );
        self.image_num = image_num as usize;

        let framebuffer = self.framebuffers[self.image_num()].clone();
        let builder = self.get_command_buffer_builder();
        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![
                        Some(background.background_color().into()),
                        Some(1f32.into()),
                    ],
                    ..RenderPassBeginInfo::framebuffer(framebuffer)
                },
                SubpassContents::Inline,
            )
            .unwrap();
    }

    pub(super) fn end_draw(&mut self) {
        let mut builder = self.get_command_buffer_builder();

        builder.end_render_pass().unwrap();

        self.execute_command_buffer();

        self.previous_frame_end = Some(
            self.previous_frame_end
                .take()
                .unwrap()
                .then_swapchain_present(
                    self.queue.clone(),
                    SwapchainPresentInfo::swapchain_image_index(
                        self.swapchain.clone(),
                        self.image_num() as u32,
                    ),
                )
                .then_signal_fence_and_flush()
                .unwrap()
                .boxed(),
        );
    }

    pub(crate) fn get_command_buffer_builder(
        &mut self,
    ) -> &mut AutoCommandBufferBuilder<
        PrimaryAutoCommandBuffer<StandardCommandBufferAlloc>,
        StandardCommandBufferAllocator,
    > {
        let queue_family_index = self.queue_family_index();
        self.command_buffer_builder.get_or_insert_with(|| {
            AutoCommandBufferBuilder::primary(
                self.command_buffer_allocator.as_ref(),
                queue_family_index,
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap()
        })
    }

    fn execute_command_buffer(&mut self) {
        if let Some(builder) = self.command_buffer_builder.take() {
            self.previous_frame_end = Some(
                self.previous_frame_end
                    .take()
                    .unwrap()
                    .then_execute(self.queue.clone(), builder.build().unwrap())
                    .unwrap()
                    .boxed(),
            );
        }
    }

    pub(crate) fn render_pass(&self) -> &Arc<RenderPass> {
        &self.render_pass
    }

    pub(crate) fn device(&self) -> &Arc<Device> {
        self.queue.device()
    }

    pub(crate) fn queue(&self) -> &Arc<Queue> {
        &self.queue
    }

    pub(crate) fn queue_family_index(&self) -> u32 {
        self.queue.queue_family_index()
    }

    pub(crate) fn swapchain(&self) -> &Arc<Swapchain> {
        &self.swapchain
    }

    pub(crate) fn image_format(&self) -> Format {
        self.swapchain.image_format()
    }

    pub(crate) fn images(&self) -> &Vec<Arc<SwapchainImage>> {
        &self.images
    }

    pub(crate) fn image_num(&self) -> usize {
        self.image_num
    }

    pub(crate) fn depth_image_format(&self) -> Format {
        self.depth_image.format()
    }

    pub(crate) fn depth_image(&self) -> &Arc<AttachmentImage> {
        &self.depth_image
    }

    pub(crate) fn current_image(&self) -> &Arc<SwapchainImage> {
        &self.images[self.image_num]
    }

    pub(crate) fn memory_allocator(&self) -> Rc<GenericMemoryAllocator<Arc<FreeListAllocator>>> {
        self.memory_allocator.clone()
    }

    pub(crate) fn descriptor_set_allocator(&self) -> Rc<StandardDescriptorSetAllocator> {
        self.descriptor_set_allocator.clone()
    }

    pub(crate) fn window(&self) -> &Window {
        self.surface
            .object()
            .unwrap()
            .downcast_ref::<Window>()
            .unwrap()
    }

    fn dimensions(&self) -> PhysicalSize<u32> {
        self.window().inner_size()
    }

    pub(crate) fn resolution(&self) -> [u32; 2] {
        self.current_image().dimensions().width_height()
    }

    pub(crate) fn resolution_f32(&self) -> [f32; 2] {
        self.resolution().map(|num| num as f32)
    }
}
