use nalgebra_glm::Vec3;
use queues::{IsQueue, Queue};

use crate::{
    public::objects::{
        background::Background,
        camera::{
            camera_2d::Camera2D,
            camera_3d::{self, Camera3D},
        },
        obj_2d::{circle::Circle, line::Line, polygon::Polygon, rect::Rect},
        obj_3d::block::Block,
    },
    rendering::render_objects::{
        background_ro::BackgroundRenderObject,
        ro_2d::{
            circle_ro::CircleRenderObject, line_ro::LineRenderObject,
            polygon_ro::PolygonRenderObject, rect_ro::RectRenderObject,
        },
        ro_3d::block_ro::BlockRenderObject,
        RenderObject2D, RenderObject3D,
    },
};

use super::{
    device_container::DeviceContainer, event_loop_container::EventLoopContainer,
    render_pass_container::RenderPassContainer,
};

pub(crate) struct RenderContainer {
    event_loop_container: Option<EventLoopContainer>,
    device_container: DeviceContainer,
    render_pass_container: RenderPassContainer,

    background: BackgroundRenderObject,
    render_objects_2d: Queue<RenderObject2D>,
    render_objects_ui: Queue<RenderObject2D>,
    render_objects_3d: Queue<RenderObject3D>,
}

impl RenderContainer {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        let event_loop_container = EventLoopContainer::new();
        let device_container =
            DeviceContainer::new(&event_loop_container.event_loop, width, height);

        let render_pass_container = RenderPassContainer::new(&device_container);

        Self {
            event_loop_container: Some(event_loop_container),
            device_container,
            render_pass_container,

            background: BackgroundRenderObject::new(Background::new_with_color(Vec3::new(
                0., 0., 0.,
            ))),
            render_objects_2d: Queue::new(),
            render_objects_ui: Queue::new(),
            render_objects_3d: Queue::new(),
        }
    }

    pub(crate) fn circle(&mut self, circle: Circle) {
        self.render_objects_2d
            .add(RenderObject2D::CircleObject(CircleRenderObject::new(
                circle,
                &mut self.device_container,
            )))
            .unwrap();
    }

    pub(crate) fn ui_circle(&mut self, circle: Circle) {
        self.render_objects_ui
            .add(RenderObject2D::CircleObject(CircleRenderObject::new(
                circle,
                &mut self.device_container,
            )))
            .unwrap();
    }

    pub(crate) fn rect(&mut self, rect: Rect) {
        self.render_objects_2d
            .add(RenderObject2D::RectObject(RectRenderObject::new(
                rect,
                &mut self.device_container,
            )))
            .unwrap();
    }

    pub(crate) fn ui_rect(&mut self, rect: Rect) {
        self.render_objects_ui
            .add(RenderObject2D::RectObject(RectRenderObject::new(
                rect,
                &mut self.device_container,
            )))
            .unwrap();
    }

    pub(crate) fn polygon(&mut self, polygon: Polygon) {
        self.render_objects_2d
            .add(RenderObject2D::PolyObject(PolygonRenderObject::new(
                polygon,
                &mut self.device_container,
            )))
            .unwrap();
    }

    pub(crate) fn ui_polygon(&mut self, polygon: Polygon) {
        self.render_objects_ui
            .add(RenderObject2D::PolyObject(PolygonRenderObject::new(
                polygon,
                &mut self.device_container,
            )))
            .unwrap();
    }

    pub(crate) fn line(&mut self, line: Line) {
        self.render_objects_2d
            .add(RenderObject2D::LineObject(LineRenderObject::new(
                line,
                &mut self.device_container,
            )))
            .unwrap();
    }

    pub(crate) fn ui_line(&mut self, line: Line) {
        self.render_objects_ui
            .add(RenderObject2D::LineObject(LineRenderObject::new(
                line,
                &mut self.device_container,
            )))
            .unwrap();
    }

    pub(crate) fn block(&mut self, block: Block) {
        self.render_objects_3d
            .add(RenderObject3D::BlockObject(BlockRenderObject::new(
                block,
                &mut self.device_container,
            )))
            .unwrap();
    }

    pub(crate) fn background(&mut self, background: Background) {
        self.background = BackgroundRenderObject::new(background);
    }

    pub(crate) fn event_loop(&mut self) -> EventLoopContainer {
        self.event_loop_container.take().unwrap()
    }

    pub(crate) fn render(&mut self, camera_2d: &Camera2D, camera_3d: &Camera3D) {
        self.device_container.begin_draw();
        
        self.render_pass_container
            .render_background(&mut self.device_container, &self.background);

        self.render_pass_container.render_3d(
            &mut self.device_container,
            &mut self.render_objects_3d,
            camera_3d,
        );

        self.render_pass_container.render_2d(
            &mut self.device_container,
            &mut self.render_objects_2d,
            camera_2d,
        );

        self.render_pass_container
            .render_ui(&mut self.device_container, &mut self.render_objects_ui);

        self.device_container.end_draw();
    }
}
