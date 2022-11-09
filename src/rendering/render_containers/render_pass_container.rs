use queues::{IsQueue, Queue};

use crate::{
    public::objects::camera::{camera_2d::Camera2D, camera_3d::Camera3D},
    rendering::{
        render_objects::{background_ro::BackgroundRenderObject, RenderObject2D, RenderObject3D},
        render_passes::{
            background_render_pass::BackgroundRenderPass,
            render_passes_2d::{
                circle_render_pass::CircleRenderPass, line_render_pass::LineRenderPass,
                poly_render_pass::PolyRenderPass,
            },
            render_passes_3d::block_render_pass::BlockRenderPass,
        },
    },
};

use super::device_container::DeviceContainer;

pub(super) struct RenderPassContainer {
    poly_render_pass: PolyRenderPass,
    circle_render_pass: CircleRenderPass,
    line_render_pass: LineRenderPass,
    background_render_pass: BackgroundRenderPass,
    block_render_pass: BlockRenderPass,
}

impl RenderPassContainer {
    pub(super) fn new(device_container: &DeviceContainer) -> Self {
        Self {
            background_render_pass: BackgroundRenderPass::new(),
            poly_render_pass: PolyRenderPass::new(&device_container),
            circle_render_pass: CircleRenderPass::new(&device_container),
            line_render_pass: LineRenderPass::new(&device_container),
            block_render_pass: BlockRenderPass::new(&device_container),
        }
    }

    pub(super) fn render_background(
        &self,
        device_container: &mut DeviceContainer,
        background_ro: &BackgroundRenderObject,
    ) {
        self.background_render_pass
            .draw(device_container, &background_ro.background.color);
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
                    block.draw(&mut self.block_render_pass, device_container, camera_3d)
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
                    rect.draw(&mut self.poly_render_pass, device_container, Some(camera_2d))
                }
                RenderObject2D::CircleObject(mut circle) => {
                    circle.draw(&mut self.circle_render_pass, device_container)
                }
                RenderObject2D::LineObject(mut line) => {
                    line.draw(&mut self.line_render_pass, device_container)
                }
                RenderObject2D::PolyObject(mut polygon) => {
                    polygon.draw(&mut self.poly_render_pass, device_container, Some(camera_2d))
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
                    rect.draw(&mut self.poly_render_pass, device_container, None)
                }
                RenderObject2D::CircleObject(mut circle) => {
                    circle.draw(&mut self.circle_render_pass, device_container)
                }
                RenderObject2D::LineObject(mut line) => {
                    line.draw(&mut self.line_render_pass, device_container)
                }
                RenderObject2D::PolyObject(mut polygon) => {
                    polygon.draw(&mut self.poly_render_pass, device_container, None)
                }
                _ => (),
            }
        }
    }
}
