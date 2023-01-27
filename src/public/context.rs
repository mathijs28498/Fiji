use std::{time::SystemTime, sync::{Arc, RwLock}};

use crate::{
    input::fiji_events::FijiEventHandler,
    public::objects::{
        background::Background, camera::camera_2d::Camera2D, obj_2d::circle::Circle,
    },
    rendering::render_containers::{
        event_loop_container::EventLoopContainer, render_container::{RenderContainer, Drawable2D, RecreateOnResize},
    },
    Input,
};

pub struct Context {
    render_container: RenderContainer,
    prev_time: SystemTime,
    dt_nano: u128,
    pub camera_2d: Camera2D,
}

impl Context {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            render_container: RenderContainer::new(width, height),
            prev_time: SystemTime::now(),
            dt_nano: 0,
            camera_2d: Camera2D::new_default(),
        }
    }

    pub fn register_recreatable(&mut self, recreatable: Arc<RwLock<dyn RecreateOnResize>>) {
        self.render_container.register_recreatable(recreatable);
    }

    pub fn draw_2d(&mut self, mut drawable: impl Drawable2D + 'static) {
        self.render_container.draw_2d(drawable);
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
            .render(fiji_event_handler, &self.camera_2d);

        let now = SystemTime::now();
        if let Ok(duration) = now.duration_since(self.prev_time) {
            self.dt_nano = duration.as_nanos();
            self.prev_time = now;
        }
    }
}
