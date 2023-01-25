use queues::{IsQueue, Queue};

use crate::{
    public::objects::camera::camera_2d::Camera2D,
    rendering::{
        pipelines::pipelines_2d::circle_pipeline::CirclePipeline,
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
}
