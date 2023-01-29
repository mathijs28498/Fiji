use std::{
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use nalgebra_glm::Vec3;
use queues::{IsQueue, Queue};

use crate::{
    input::fiji_events::FijiEventHandler,
    public::objects::{
        background::Background, camera::camera_2d::Camera2D, obj_2d::circle::Circle,
    },
    rendering::pipelines::pipelines_2d::circle_pipeline::{CirclePipeline, CIRCLE_PIPELINE},
    Context, Input,
};

use super::{device_container::DeviceContainer, event_loop_container::EventLoopContainer};

pub trait RecreateOnResize {
    fn recreate(&mut self, device_container: &mut DeviceContainer);
}

pub trait Drawable2D {
    fn draw(&mut self, device_container: &mut DeviceContainer, camera_2d: Option<&Camera2D>);
    // TODO: Remove this at some point
    fn set_buffers(&mut self, device_container: &mut DeviceContainer);
}

pub(crate) struct RenderContainer {
    event_loop_container: Option<EventLoopContainer>,
    device_container: DeviceContainer,

    recreatables: Vec<Arc<RwLock<dyn RecreateOnResize>>>,
    background: Background,
    drawables_2d: Vec<Box<dyn Drawable2D>>,
}

impl RenderContainer {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        let event_loop_container = EventLoopContainer::new();
        let device_container =
            DeviceContainer::new(&event_loop_container.event_loop, width, height);

        let recreatables = Self::init_recreatables(&device_container);
        Self {
            event_loop_container: Some(event_loop_container),
            device_container,

            recreatables,
            background: Background::new_with_color(Vec3::new(0., 0., 0.)),
            drawables_2d: Vec::new(),
        }
    }

    fn init_recreatables(
        device_container: &DeviceContainer,
    ) -> Vec<Arc<RwLock<dyn RecreateOnResize>>> {
        vec![CIRCLE_PIPELINE
            .get_or_init(|| Arc::new(RwLock::new(CirclePipeline::new(device_container))))
            .clone()]
    }

    pub(crate) fn register_recreatable(&mut self, recreatable: Arc<RwLock<dyn RecreateOnResize>>) {
        self.recreatables.push(recreatable)
    }

    pub(crate) fn draw_2d(&mut self, mut drawable: impl Drawable2D + 'static) {
        drawable.set_buffers(&mut self.device_container);
        self.drawables_2d.push(Box::new(drawable));
    }

    pub(crate) fn background(&mut self, background: Background) {
        self.background = background;
    }

    pub(crate) fn event_loop(&mut self) -> EventLoopContainer {
        self.event_loop_container.take().unwrap()
    }

    pub(crate) fn render(
        &mut self,
        fiji_event_handler: &mut FijiEventHandler,
        camera_2d: &Camera2D,
    ) {
        if fiji_event_handler.recreate_pipelines {
            if !self.device_container.recreate_swapchain_images() {
                return;
            }

            for recreatable in &self.recreatables {
                // TODO: write while loop until can unwrap
                let mut recreatable_lock = recreatable.write().unwrap();
                recreatable_lock.recreate(&mut self.device_container);
            }
        }

        self.device_container.begin_draw(&self.background);

        // while let Some(drawable) = self.drawables_2d.remove() {
        //     drawable.draw(&mut self.device_container, Some(camera_2d))
        // }
        for drawable in self.drawables_2d.iter_mut() {
            drawable.draw(&mut self.device_container, Some(camera_2d))
        }
        // self.pipeline_container.render_2d(
        //     &mut self.device_container,
        //     &mut self.render_objects_2d,
        //     camera_2d,
        // );

        self.device_container.end_draw();
    }
}
