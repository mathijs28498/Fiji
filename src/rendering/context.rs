use nalgebra_glm::Vec3;

use crate::objects::camera::camera_2d::Camera2D;
use crate::objects::camera::camera_3d::Camera3D;
use crate::objects::obj_2d::circle::Circle;
use crate::objects::obj_2d::line::Line;
use crate::objects::obj_2d::polygon::Polygon;
use crate::objects::obj_3d::block::Block;
use crate::objects::{background::Background, obj_2d::rect::Rect, DrawObject2D, DrawObject3D};

use super::render_passes::block_render_pass::BlockRenderPass;
use super::render_passes::circle_render_pass::CircleRenderPass;
use super::render_passes::line_render_pass::LineRenderPass;
use super::{
    device_container::DeviceContainer,
    event_loop_container::EventLoopContainer,
    render_passes::{
        background_render_pass::BackgroundRenderPass, poly_render_pass::PolyRenderPass,
    },
};

pub struct Context {
    event_loop_container: Option<EventLoopContainer>,
    device_container: DeviceContainer,
    poly_render_pass: PolyRenderPass,
    circle_render_pass: CircleRenderPass,
    line_render_pass: LineRenderPass,
    background_render_pass: BackgroundRenderPass,
    block_render_pass: BlockRenderPass,
    pub camera_2d: Camera2D,
    pub camera_3d: Camera3D,

    // TODO: Split objects into 2d and 3d
    background: Background,
    draw_objects_2d: Vec<DrawObject2D>,
    draw_objects_3d: Vec<DrawObject3D>,
}

impl Context {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop_container = EventLoopContainer::new();
        let device_container =
            DeviceContainer::new(&event_loop_container.event_loop, width, height);

        let background = Background::new(Vec3::new(0., 0., 0.));
        let background_render_pass = BackgroundRenderPass::new();

        let poly_render_pass = PolyRenderPass::new(&device_container);
        let circle_render_pass = CircleRenderPass::new(&device_container);
        let line_render_pass = LineRenderPass::new(&device_container);
        let block_render_pass = BlockRenderPass::new(&device_container);

        Self {
            event_loop_container: Some(event_loop_container),
            device_container,
            poly_render_pass,
            circle_render_pass,
            line_render_pass,
            background_render_pass,
            block_render_pass,

            camera_2d: Camera2D {},
            camera_3d: Camera3D::new(),

            background,
            draw_objects_2d: Vec::new(),
            draw_objects_3d: Vec::new(),
        }
    }

    fn draw_2d(&mut self, draw_object: DrawObject2D) {
        self.draw_objects_2d.push(draw_object);
    }

    fn draw_3d(&mut self, draw_object: DrawObject3D) {
        self.draw_objects_3d.push(draw_object);
    }

    pub fn draw_circle(&mut self, circle: Circle) {
        self.draw_2d(DrawObject2D::CircleObject(circle));
    }

    pub fn draw_rect(&mut self, rect: Rect) {
        self.draw_2d(DrawObject2D::RectObject(rect));
    }

    pub fn draw_polygon(&mut self, polygon: Polygon) {
        self.draw_2d(DrawObject2D::PolyObject(polygon));
    }

    pub fn draw_line(&mut self, line: Line) {
        self.draw_2d(DrawObject2D::LineObject(line));
    }

    pub fn draw_block(&mut self, block: Block) {
        self.draw_3d(DrawObject3D::BlockObject(block));
    }

    pub fn draw_background(&mut self, bg: Background) {
        self.background = bg;
    }

    pub fn event_loop(&mut self) -> EventLoopContainer {
        self.event_loop_container.take().unwrap()
    }

    pub fn render(&mut self) {
        self.device_container.begin_draw();

        self.background
            .draw(&self.background_render_pass, &mut self.device_container);

        for object in self.draw_objects_3d.iter_mut() {
            match object {
                DrawObject3D::BlockObject(block) => block.draw(
                    &mut self.block_render_pass,
                    &mut self.device_container,
                    &self.camera_3d,
                ),
            }
        };

        for object in self.draw_objects_2d.iter_mut() {
            match object {
                DrawObject2D::RectObject(rect) => {
                    rect.draw(&mut self.poly_render_pass, &mut self.device_container)
                }
                DrawObject2D::CircleObject(circle) => {
                    circle.draw(&mut self.circle_render_pass, &mut self.device_container)
                }
                DrawObject2D::LineObject(line) => {
                    line.draw(&mut self.line_render_pass, &mut self.device_container)
                }
                DrawObject2D::PolyObject(polygon) => {
                    polygon.draw(&mut self.poly_render_pass, &mut self.device_container)
                }
            }
        }
        self.device_container.end_draw();

        self.clear_objects();
    }

    fn clear_objects(&mut self) {
        self.draw_objects_2d = Vec::new();
        self.draw_objects_3d = Vec::new();
    }
}
