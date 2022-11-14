use std::sync::Arc;

use vulkano::{
    command_buffer::{allocator::StandardCommandBufferAllocator, PrimaryAutoCommandBuffer},
    descriptor_set::allocator::{DescriptorSetAllocator, StandardDescriptorSetAllocator},
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue,
        QueueCreateInfo,
    },
    format::Format,
    image::{AttachmentImage, ImageAccess, ImageUsage, SwapchainImage},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{FreeListAllocator, GenericMemoryAllocator, StandardMemoryAllocator},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
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

use crate::input::fiji_events::FijiEventHandler;

pub(crate) struct DeviceContainer {
    surface: Arc<Surface>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    depth_image: Arc<AttachmentImage>,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    image_num: usize,

    memory_allocator: GenericMemoryAllocator<Arc<FreeListAllocator>>,
    command_buffer_allocator: StandardCommandBufferAllocator,
    descriptor_set_allocator: StandardDescriptorSetAllocator,
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

        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();
        {
            let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();

            window.set_inner_size(PhysicalSize::new(width, height));
        }

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

        let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(device.clone(), Default::default());
        let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());

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
            &memory_allocator,
            images[0].dimensions().width_height(),
            Format::D32_SFLOAT,
            ImageUsage {
                transfer_dst: true,
                ..ImageUsage::empty()
            },
        )
        .unwrap();

        let previous_frame_end = Some(sync::now(queue.device().clone()).boxed());

        Self {
            surface,
            queue,
            swapchain,
            images,
            depth_image,
            previous_frame_end,
            image_num: 0,
            memory_allocator,
            command_buffer_allocator,
            descriptor_set_allocator,
        }
    }

    pub(super) fn recreate_swapchain_images(&mut self) {
        (self.swapchain, self.images) = self
            .swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: self.dimensions().into(),
                ..self.swapchain.create_info()
            })
            .unwrap();

        self.depth_image = AttachmentImage::with_usage(
            self.memory_allocator(),
            self.resolution(),
            Format::D32_SFLOAT,
            ImageUsage {
                transfer_dst: true,
                ..ImageUsage::empty()
            },
        )
        .unwrap();
    }

    pub(super) fn begin_draw(&mut self) {
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
    }

    pub(super) fn end_draw(&mut self) {
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

    pub(crate) fn execute_command_buffer(&mut self, command_buffer: PrimaryAutoCommandBuffer) {
        self.previous_frame_end = Some(
            self.previous_frame_end
                .take()
                .unwrap()
                .then_execute(self.queue().clone(), command_buffer)
                .unwrap()
                .boxed(),
        )
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

    pub(crate) fn memory_allocator(&self) -> &GenericMemoryAllocator<Arc<FreeListAllocator>> {
        &self.memory_allocator
    }

    pub(crate) fn command_buffer_allocator(&self) -> &StandardCommandBufferAllocator {
        &self.command_buffer_allocator
    }

    pub(crate) fn descriptor_set_allocator(&self) -> &StandardDescriptorSetAllocator {
        &self.descriptor_set_allocator
    }

    pub(crate) fn window(&self) -> &Window {
        self.surface
            .object()
            .unwrap()
            .downcast_ref::<Window>()
            .unwrap()
    }

    pub(crate) fn dimensions(&self) -> PhysicalSize<u32> {
        self.window().inner_size()
    }

    pub(crate) fn resolution(&self) -> [u32; 2] {
        self.current_image().dimensions().width_height()
    }

    pub(crate) fn resolution_f32(&self) -> [f32; 2] {
        self.resolution().map(|num| num as f32)
    }
}
