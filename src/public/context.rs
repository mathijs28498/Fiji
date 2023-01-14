use std::time::SystemTime;

use crate::{
    input::fiji_events::FijiEventHandler,
    public::objects::{
        background::Background,
        camera::{camera_2d::Camera2D, camera_3d::Camera3D},
        obj_2d::{circle::Circle, line::Line, polygon::Polygon, rect::Rect},
        obj_3d::block::Block,
    },
    rendering::render_containers::{
        event_loop_container::EventLoopContainer, render_container::RenderContainer,
    },
    Figure, Input,
};

use super::objects::obj_2d::text::Text;

pub struct Context {
    render_container: RenderContainer,
    prev_time: SystemTime,
    dt_nano: u128,
    pub camera_2d: Camera2D,
    pub camera_3d: Camera3D,
}

impl Context {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            render_container: RenderContainer::new(width, height),
            prev_time: SystemTime::now(),
            dt_nano: 0,
            camera_2d: Camera2D::new_default(),
            camera_3d: Camera3D::new_default(),
        }
    }

    pub fn circle(&mut self, circle: Circle) {
        self.render_container.circle(circle);
    }

    pub fn ui_circle(&mut self, circle: Circle) {
        self.render_container.ui_circle(circle);
    }

    pub fn rect(&mut self, rect: Rect) {
        self.render_container.rect(rect);
    }

    pub fn ui_rect(&mut self, rect: Rect) {
        self.render_container.ui_rect(rect);
    }

    pub fn polygon(&mut self, polygon: Polygon) {
        self.render_container.polygon(polygon);
    }

    pub fn ui_polygon(&mut self, polygon: Polygon) {
        self.render_container.ui_polygon(polygon);
    }

    pub fn line(&mut self, line: Line) {
        self.render_container.line(line);
    }

    pub fn ui_line(&mut self, line: Line) {
        self.render_container.ui_line(line);
    }

    pub fn text(&mut self, text: Text) {
        self.render_container.text(text);
    }

    pub fn ui_text(&mut self, text: Text) {
        self.render_container.ui_text(text);
    }

    pub fn figure(&mut self, figure: Figure) {
        self.render_container.figure(figure);
    }

    pub fn ui_figure(&mut self, figure: Figure) {
        self.render_container.ui_figure(figure);
    }

    pub fn block(&mut self, block: Block) {
        self.render_container.block(block);
    }

    pub fn background(&mut self, background: Background) {
        self.render_container.background(background);
    }

    pub fn run<F>(mut self, mut event_fn: F)
    where
        F: 'static + FnMut(&Input, &mut FijiEventHandler, &mut Context),
    {
        self.render_container.event_loop().run(self, event_fn);
    }

    pub fn fps(&self) -> f32 {
        1. / self.dt()
    }

    pub fn dt(&self) -> f32 {
        self.dt_nano as f32 * 0.000000001
    }

    pub fn dt_f64(&self) -> f64 {
        self.dt_nano as f64 * 0.000000001
    }

    pub fn dt_micros(&self) -> u128 {
        self.dt_nano / 1_000
    }

    pub fn dt_millis(&self) -> u128 {
        self.dt_nano / 1_000_000
    }

    pub fn render(&mut self, fiji_event_handler: &mut FijiEventHandler) {
        self.render_container
            .render(fiji_event_handler, &self.camera_2d, &self.camera_3d);

        let now = SystemTime::now();
        if let Ok(duration) = now.duration_since(self.prev_time) {
            self.dt_nano = duration.as_nanos();
            self.prev_time = now;
        }
    }
}
