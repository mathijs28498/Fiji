use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::{event::Event, event_loop::ControlFlow};

use crate::draw_objects::circle::Circle;
use crate::draw_objects::{background::Background, square::Square, DrawObject};

use super::render_passes::circle_render_pass::CircleRenderPass;
use super::{
    device_container::DeviceContainer,
    render_passes::{
        background_render_pass::BackgroundRenderPass, poly_render_pass::PolyRenderPass,
    },
};

pub struct EventLoopContainer {
    event_loop: EventLoop<()>,
}

impl EventLoopContainer {
    fn new(event_loop: EventLoop<()>) -> EventLoopContainer {
        EventLoopContainer { event_loop }
    }

    pub fn run<F>(self, mut event_handler: F)
    where
        F: 'static + FnMut(),
    {
        self.event_loop.run(
            move |event, _, control_flow: &mut ControlFlow| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::RedrawEventsCleared => {
                    event_handler();
                }
                _ => (),
            },
        );
    }
}

pub struct Context {
    event_loop_container: Option<EventLoopContainer>,
    device_container: DeviceContainer,
    poly_render_pass: PolyRenderPass,
    circle_render_pass: CircleRenderPass,
    background_render_pass: BackgroundRenderPass,
    draw_objects: Vec<DrawObject>,
}

impl Context {
    pub fn new() -> Self {
        let event_loop_container = EventLoopContainer::new(EventLoop::new());
        let device_container = DeviceContainer::new(&event_loop_container.event_loop);
        let poly_render_pass = PolyRenderPass::new(&device_container);
        let circle_render_pass = CircleRenderPass::new(&device_container);
        let background_render_pass = BackgroundRenderPass::new();

        Self {
            event_loop_container: Some(event_loop_container),
            device_container,
            poly_render_pass,
            circle_render_pass,
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

    pub fn draw_square(&mut self, square: Square) {
        self.draw(DrawObject::SquareObject(square));
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
                DrawObject::SquareObject(square) => {
                    square.draw(&mut self.poly_render_pass, &mut self.device_container);
                }
                DrawObject::CircleObject(circle) => {
                    circle.draw(&mut self.circle_render_pass, &mut self.device_container);
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
