use crate::{
    public::objects::{
        background::Background,
        camera::{camera_2d::Camera2D, camera_3d::Camera3D},
        obj_2d::{circle::Circle, line::Line, polygon::Polygon, rect::Rect},
        obj_3d::block::Block,
    },
    rendering::render_containers::{
        event_loop_container::EventLoopContainer, render_container::RenderContainer,
    },
};

pub struct Context {
    render_container: RenderContainer,
    pub camera_2d: Camera2D,
    pub camera_3d: Camera3D,
}

impl Context {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            render_container: RenderContainer::new(width, height),
            camera_2d: Camera2D::new(),
            camera_3d: Camera3D::new(),
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

    pub fn block(&mut self, block: Block) {
        self.render_container.block(block);
    }

    pub fn background(&mut self, background: Background) {
        self.render_container.background(background);
    }

    pub fn event_loop(&mut self) -> EventLoopContainer {
        self.render_container.event_loop()
    }

    pub fn render(&mut self) {
        self.render_container
            .render(&self.camera_2d, &self.camera_3d);
    }
}
