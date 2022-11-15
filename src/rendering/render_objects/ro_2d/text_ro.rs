use crate::{
    public::objects::{camera::camera_2d::Camera2D, obj_2d::text::Text},
    rendering::{
        pipelines::pipelines_2d::text_pipeline::{text_fs, TextPipeline},
        render_containers::device_container::DeviceContainer,
    },
};

#[derive(Clone)]
pub(crate) struct TextRenderObject {
    text: Text,
}

impl TextRenderObject {
    pub(crate) fn new(text: Text) -> Self {
        Self { text }
    }

    #[allow(non_snake_case)]
    pub(crate) fn draw(
        &mut self,
        text_pipeline: &mut TextPipeline,
        device_container: &mut DeviceContainer,
        camera_2d: Option<&Camera2D>,
    ) {
        let cameraPos = match camera_2d {
            Some(camera_2d) => camera_2d.position.as_ref().clone(),
            None => [0.; 2],
        };

        let sets = self
            .text
            .text
            .chars()
            .map(|c| text_pipeline.get_or_create_set(device_container, c, &self.text.font))
            .collect::<Vec<_>>();

        text_pipeline.draw(
            device_container,
            text_fs::ty::Constants {
                resolution: device_container.resolution(),
                position: self.text.position.as_ref().clone(),
                color: self.text.color.as_ref().clone(),
                cameraPos,
            },
            sets,
        );
    }
}
