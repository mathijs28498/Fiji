use queues::{IsQueue, Queue};

use crate::{
    public::objects::camera::camera_2d::Camera2D,
    rendering::{
        pipelines::pipelines_2d::circle_pipeline::CirclePipeline,
        render_objects::{background_ro::BackgroundRenderObject, RenderObject2D},
    },
};

use super::device_container::DeviceContainer;

pub(super) struct PipelineContainer {
    circle_pipeline: CirclePipeline,
}

impl PipelineContainer {
    pub(super) fn new(device_container: &DeviceContainer) -> Self {
        Self {
            circle_pipeline: CirclePipeline::new(device_container),
        }
    }

    // TODO: Add line pipeline recreation
    pub(super) fn recreate_pipelines(&mut self, device_container: &DeviceContainer) {
        self.circle_pipeline.recreate_pipeline(device_container);
    }

    pub(super) fn render_2d(
        &mut self,
        device_container: &mut DeviceContainer,
        render_objects: &mut Queue<RenderObject2D>,
        camera_2d: &Camera2D,
    ) {
        while let Ok(object) = render_objects.remove() {
            match object {
                RenderObject2D::CircleObject(mut circle) => {
                    circle.draw(&mut self.circle_pipeline, device_container, Some(camera_2d))
                }
            }
        }
    }
}
