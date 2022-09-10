use winit::event_loop::EventLoop;

use crate::draw_objects::{DrawObject, square::Square, background::Background};

use super::{
    device_container::DeviceContainer,
    render_passes::{
        background_render_pass::BackgroundRenderPass, poly_render_pass::PolyRenderPass,
    },
};

pub struct Context {
    pub event_loop: Option<EventLoop<()>>,
    device_container: DeviceContainer,
    poly_render_pass: PolyRenderPass,
    background_render_pass: BackgroundRenderPass,
    draw_objects: Vec<DrawObject>,
}

impl Context {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let device_container = DeviceContainer::new(&event_loop);
        let poly_render_pass = PolyRenderPass::new(&device_container);
        let background_render_pass = BackgroundRenderPass::new();

        Self {
            event_loop: Some(event_loop),
            device_container,
            poly_render_pass,
            background_render_pass,
            draw_objects: Vec::new(),
        }
    }

    pub fn draw(&mut self, draw_object: DrawObject) {
        self.draw_objects.push(draw_object);
    }

    pub fn draw_square(&mut self, square: Square) {
        self.draw(DrawObject::SquareObject(square));
    }

    pub fn draw_background(&mut self, bg: Background) {
        self.draw(DrawObject::BackgroundObject(bg));
    }

    pub fn render(&mut self) {
        self.device_container.begin_draw();

        for object in self.draw_objects.iter_mut() {
            match object {
                DrawObject::SquareObject(square) => {
                    square.draw(&mut self.poly_render_pass, &mut self.device_container);
                }
                DrawObject::BackgroundObject(bg) => {
                    bg.draw(&self.background_render_pass, &mut self.device_container);
                }
            }
            // match object.render_pass_type() {
            //     RenderPassType::Poly => {
            //         // TODO: Change this so that the drawing is done in the object function
            //         // TODO: Possibly by creating another enum that holds different renderpasses
            //         let (vb, ib) = object.get_buffers(self.device_container.queue());
            //         self.poly_render_pass
            //             .draw(&mut self.device_container, vb.clone(), ib.clone());
            //     }
            // }
        }
        self.device_container.end_draw();

        self.clear_objects();
    }

    fn clear_objects(&mut self) {
        self.draw_objects = Vec::new();
    }
}
