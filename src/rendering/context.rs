use crate::draw_objects::circle::Circle;
use crate::draw_objects::line::Line;
use crate::draw_objects::polygon::Polygon;
use crate::draw_objects::{background::Background, rect::Rect, DrawObject};

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
    draw_objects: Vec<DrawObject>,
}

impl Context {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop_container = EventLoopContainer::new();
        let device_container = DeviceContainer::new(&event_loop_container.event_loop, width, height);
        let poly_render_pass = PolyRenderPass::new(&device_container);
        let circle_render_pass = CircleRenderPass::new(&device_container);
        let line_render_pass = LineRenderPass::new(&device_container);
        let background_render_pass = BackgroundRenderPass::new();

        Self {
            event_loop_container: Some(event_loop_container),
            device_container,
            poly_render_pass,
            circle_render_pass,
            line_render_pass,
            background_render_pass,
            draw_objects: Vec::new(),
        }
    }

    pub fn draw(&mut self, draw_object: DrawObject) {
        self.draw_objects.push(draw_object);
    }

    pub fn draw_circle(&mut self, circle: Circle) {
        self.draw(DrawObject::CircleObject(circle));
    }

    pub fn draw_rect(&mut self, rect: Rect) {
        self.draw(DrawObject::RectObject(rect));
    }

    pub fn draw_polygon(&mut self, polygon: Polygon) {
        self.draw(DrawObject::PolyObject(polygon));
    }

    pub fn draw_line(&mut self, line: Line) {
        self.draw(DrawObject::LineObject(line));
    }

    pub fn draw_background(&mut self, bg: Background) {
        self.draw(DrawObject::BackgroundObject(bg));
    }

    pub fn event_loop(&mut self) -> EventLoopContainer {
        self.event_loop_container.take().unwrap()
    }

    pub fn render(&mut self) {
        self.device_container.begin_draw();

        for object in self.draw_objects.iter_mut() {
            match object {
                DrawObject::RectObject(rect) => {
                    rect.draw(&mut self.poly_render_pass, &mut self.device_container);
                }
                DrawObject::CircleObject(circle) => {
                    circle.draw(&mut self.circle_render_pass, &mut self.device_container);
                }
                DrawObject::LineObject(line) => {
                    line.draw(&mut self.line_render_pass, &mut self.device_container)
                }
                DrawObject::PolyObject(polygon) => {
                    polygon.draw(&mut self.poly_render_pass, &mut self.device_container)
                }
                DrawObject::BackgroundObject(bg) => {
                    bg.draw(&self.background_render_pass, &mut self.device_container);
                }
            }
        }
        self.device_container.end_draw();

        self.clear_objects();
    }

    fn clear_objects(&mut self) {
        self.draw_objects = Vec::new();
    }
}
