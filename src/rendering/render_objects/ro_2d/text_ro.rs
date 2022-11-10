use crate::{
    public::objects::{camera::camera_2d::Camera2D, obj_2d::text::Text},
    rendering::render_containers::device_container::DeviceContainer,
};

#[derive(Clone)]
pub(crate) struct TextRenderObject {
    text: Text,
}

impl TextRenderObject {
    pub(crate) fn new(text: Text, device_container: &mut DeviceContainer) -> Self {
        Self { text }
    }

    pub(crate) fn draw(&mut self, camera_2d: Option<&Camera2D>) {}
}
