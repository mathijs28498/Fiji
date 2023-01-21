use nalgebra_glm::Vec3;
use queues::{IsQueue, Queue};

use crate::{
    input::fiji_events::FijiEventHandler,
    public::objects::{
        background::Background, camera::camera_2d::Camera2D, obj_2d::circle::Circle,
    },
    rendering::render_objects::{
        background_ro::BackgroundRenderObject, ro_2d::circle_ro::CircleRenderObject, RenderObject2D,
    },
    Context, Input,
};

use super::{
    device_container::DeviceContainer, event_loop_container::EventLoopContainer,
    pipeline_container::PipelineContainer,
};

pub(crate) struct RenderContainer {
    event_loop_container: Option<EventLoopContainer>,
    device_container: DeviceContainer,
    pipeline_container: PipelineContainer,

    background: BackgroundRenderObject,
    render_objects_2d: Queue<RenderObject2D>,
}

impl RenderContainer {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        let event_loop_container = EventLoopContainer::new();
        let device_container =
            DeviceContainer::new(&event_loop_container.event_loop, width, height);

        let pipeline_container = PipelineContainer::new(&device_container);

        Self {
            event_loop_container: Some(event_loop_container),
            device_container,
            pipeline_container,

            background: BackgroundRenderObject::new(Background::new_with_color(Vec3::new(
                0., 0., 0.,
            ))),
            render_objects_2d: Queue::new(),
        }
    }

    pub(crate) fn circle(&mut self, circle: Circle) {
        self.render_objects_2d
            .add(RenderObject2D::CircleObject(CircleRenderObject::new(
                circle,
                &mut self.device_container,
            )))
            .unwrap();
    }

    pub(crate) fn background(&mut self, background: Background) {
        self.background = BackgroundRenderObject::new(background);
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
            self.pipeline_container
                .recreate_pipelines(&self.device_container);

            // TODO: Check if this is necessary
            fiji_event_handler.recreate_pipelines = false;
        }

        self.device_container.begin_draw(&self.background);

        self.pipeline_container.render_2d(
            &mut self.device_container,
            &mut self.render_objects_2d,
            camera_2d,
        );

        self.device_container.end_draw();
    }
}
