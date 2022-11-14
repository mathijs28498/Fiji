use queues::{IsQueue, Queue};

use crate::{
    public::objects::camera::{camera_2d::Camera2D, camera_3d::Camera3D},
    rendering::{
        pipelines::{
            background_pipeline::BackgroundRenderPass,
            pipelines_2d::{
                circle_pipeline::CirclePipeline, line_pipeline::LinePipeline,
                poly_pipeline::PolyPipeline, text_pipeline::TextPipeline,
            },
            pipelines_3d::block_pipeline::BlockPipeline,
        },
        render_objects::{background_ro::BackgroundRenderObject, RenderObject2D, RenderObject3D},
    },
};

use super::device_container::DeviceContainer;

pub(super) struct PipelineContainer {
    poly_pipeline: PolyPipeline,
    circle_pipeline: CirclePipeline,
    line_pipeline: LinePipeline,
    background_pipeline: BackgroundRenderPass,
    block_pipeline: BlockPipeline,
    text_pipeline: TextPipeline,
}

impl PipelineContainer {
    pub(super) fn new(device_container: &DeviceContainer) -> Self {
        Self {
            background_pipeline: BackgroundRenderPass::new(),
            poly_pipeline: PolyPipeline::new(device_container),
            circle_pipeline: CirclePipeline::new(device_container),
            line_pipeline: LinePipeline::new(device_container),
            block_pipeline: BlockPipeline::new(device_container),
            text_pipeline: TextPipeline::new(device_container),
        }
    }

    pub(super) fn render_background(
        &mut self,
        device_container: &mut DeviceContainer,
        background_ro: &BackgroundRenderObject,
    ) {
        background_ro.draw(&mut self.background_pipeline, device_container);
    }

    pub(super) fn render_3d(
        &mut self,
        device_container: &mut DeviceContainer,
        render_objects: &mut Queue<RenderObject3D>,
        camera_3d: &Camera3D,
    ) {
        while let Ok(object) = render_objects.remove() {
            match object {
                RenderObject3D::BlockObject(mut block) => {
                    block.draw(&mut self.block_pipeline, device_container, camera_3d)
                }
            }
        }
    }

    pub(super) fn render_2d(
        &mut self,
        device_container: &mut DeviceContainer,
        render_objects: &mut Queue<RenderObject2D>,
        camera_2d: &Camera2D,
    ) {
        while let Ok(object) = render_objects.remove() {
            match object {
                RenderObject2D::RectObject(mut rect) => {
                    rect.draw(&mut self.poly_pipeline, device_container, Some(camera_2d))
                }
                RenderObject2D::CircleObject(mut circle) => {
                    circle.draw(&mut self.circle_pipeline, device_container)
                }
                RenderObject2D::LineObject(mut line) => {
                    line.draw(&mut self.line_pipeline, device_container)
                }
                RenderObject2D::PolyObject(mut polygon) => {
                    polygon.draw(&mut self.poly_pipeline, device_container, Some(camera_2d))
                }
                RenderObject2D::TextObject(mut text) => {
                    text.draw(&mut self.text_pipeline, device_container, Some(camera_2d))
                }
            }
        }
    }

    pub(super) fn render_ui(
        &mut self,
        device_container: &mut DeviceContainer,
        render_objects: &mut Queue<RenderObject2D>,
    ) {
        while let Ok(object) = render_objects.remove() {
            match object {
                RenderObject2D::RectObject(mut rect) => {
                    rect.draw(&mut self.poly_pipeline, device_container, None)
                }
                RenderObject2D::CircleObject(mut circle) => {
                    circle.draw(&mut self.circle_pipeline, device_container)
                }
                RenderObject2D::LineObject(mut line) => {
                    line.draw(&mut self.line_pipeline, device_container)
                }
                RenderObject2D::PolyObject(mut polygon) => {
                    polygon.draw(&mut self.poly_pipeline, device_container, None)
                }
                RenderObject2D::TextObject(mut text) => {
                    text.draw(&mut self.text_pipeline, device_container, None)
                }
            }
        }
    }
}
