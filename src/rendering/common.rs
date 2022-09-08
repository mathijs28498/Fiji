use std::sync::Arc;
use vulkano::{
    buffer::CpuAccessibleBuffer,
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo,
    },
    format::Format,
    image::{self, ImageUsage, SwapchainImage},
    instance::{Instance, InstanceCreateInfo},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo,
    },
    sync,
    sync::GpuFuture,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use super::{data_types::Vertex, render_passes::PolyRenderPass};

pub(crate) struct DeviceContainer {
    queue: Arc<Queue>,
    surface: Arc<Surface<Window>>,
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    pub(crate) previous_frame_end: Option<Box<dyn GpuFuture>>,
    image_num: usize,
}

impl DeviceContainer {
    pub(crate) fn new(event_loop: &EventLoop<()>) -> Self {
        let required_extensions = vulkano_win::required_extensions();

        let instance = Instance::new(InstanceCreateInfo {
            enabled_extensions: required_extensions,
            enumerate_portability: true,
            ..Default::default()
        })
        .unwrap();

        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };

        let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
            .filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
            .filter_map(|p| {
                p.queue_families()
                    .find(|&q| {
                        q.supports_graphics() && q.supports_surface(&surface).unwrap_or(false)
                    })
                    .map(|q| (p, q))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
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
                queue_create_infos: vec![QueueCreateInfo::family(queue_family)],

                ..Default::default()
            },
        )
        .unwrap();

        let queue = queues.next().unwrap();
        let (swapchain, images) = {
            let surface_capabilities = physical_device
                .surface_capabilities(&surface, Default::default())
                .unwrap();

            let image_format = Some(
                physical_device
                    .surface_formats(&surface, Default::default())
                    .unwrap()[0]
                    .0,
            );

            Swapchain::new(
                device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count,
                    image_format,
                    image_extent: surface.window().inner_size().into(),
                    image_usage: ImageUsage {
                        ..ImageUsage::color_attachment()
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

        let previous_frame_end = Some(sync::now(queue.device().clone()).boxed());

        Self {
            queue,
            surface,
            swapchain,
            images,
            previous_frame_end,
            image_num: 0,
        }
    }

    fn begin_draw(&mut self) {
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();
        let (image_num, suboptimal, acquire_future) =
            acquire_next_image(self.swapchain().clone(), None).unwrap();

        self.previous_frame_end = Some(
            self.previous_frame_end
                .take()
                .unwrap()
                .join(acquire_future)
                .boxed(),
        );
        self.image_num = image_num;
    }

    fn end_draw(&mut self) {
        self.previous_frame_end = Some(
            self.previous_frame_end
                .take()
                .unwrap()
                .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), self.image_num)
                .then_signal_fence_and_flush()
                .unwrap()
                .boxed(),
        );
    }

    pub(crate) fn device(&self) -> &Arc<Device> {
        self.queue.device()
    }

    pub(crate) fn queue(&self) -> &Arc<Queue> {
        &self.queue
    }

    pub(crate) fn swapchain(&self) -> &Arc<Swapchain<Window>> {
        &self.swapchain
    }

    pub(crate) fn image_format(&self) -> Format {
        self.swapchain.image_format()
    }

    pub(crate) fn images(&self) -> &Vec<Arc<SwapchainImage<Window>>> {
        &self.images
    }

    pub(crate) fn image_num(&self) -> usize {
        self.image_num
    }
}

pub(crate) struct Context {
    device_container: DeviceContainer,
    poly_render_pass: PolyRenderPass,
    vertex_buffers: Vec<Arc<CpuAccessibleBuffer<[Vertex]>>>,
}

impl Context {
    pub(crate) fn new(event_loop: &EventLoop<()>) -> Self {
        let device_container = DeviceContainer::new(event_loop);
        let poly_render_pass = PolyRenderPass::new(&device_container);

        Self {
            device_container,
            poly_render_pass,
            vertex_buffers: Vec::new(),
        }
    }

    pub(crate) fn draw_vertex_buffer(
        &mut self,
        vertex_buffer: &Arc<CpuAccessibleBuffer<[Vertex]>>,
    ) {
        self.vertex_buffers.push(vertex_buffer.clone());
    }

    pub(crate) fn render(&mut self) {
        self.device_container.begin_draw();
        for vb in &self.vertex_buffers {
            self.poly_render_pass
                .draw(&mut self.device_container, vb.clone());
        }
        self.device_container.end_draw();

        self.clear_objects();
    }

    fn clear_objects(&mut self) {
        self.vertex_buffers = Vec::new();
    }

    pub(crate) fn device(&self) -> &Arc<Device> {
        self.device_container.device()
    }
}
