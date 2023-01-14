use crate::{
    public::objects::background::Background,
    rendering::render_containers::device_container::DeviceContainer,
};

pub(crate) struct BackgroundRenderObject {
    background: Background,
}

impl BackgroundRenderObject {
    pub(crate) fn new(background: Background) -> Self {
        Self { background }
    }

    pub fn background_color(&self) -> [f32; 4] {
        [
            self.background.color.x,
            self.background.color.y,
            self.background.color.z,
            1.,
        ]
    }
}
