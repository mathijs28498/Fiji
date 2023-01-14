use std::sync::Arc;

use fontdue::Metrics;
use vulkano::descriptor_set::PersistentDescriptorSet;

use crate::{
    public::objects::{camera::camera_2d::Camera2D, obj_2d::text::Text},
    rendering::{
        pipelines::pipelines_2d::text_pipeline::{text_fs, TextPipeline},
        render_containers::device_container::DeviceContainer,
        render_objects::shared::{create_buffers_2d_uv, BufferContainer2DUv, Vertex2DUv},
    },
};

#[derive(Clone)]
pub(crate) struct TextRenderObject {
    text: Text,
    sets: Vec<Arc<PersistentDescriptorSet>>,
    buffers_vec: Vec<BufferContainer2DUv>,
}

impl TextRenderObject {
    // TODO: Do this differently!!! Don't require the pipeline and devicecontainer
    pub(crate) fn new(
        text: Text,
        text_pipeline: &mut TextPipeline,
        device_container: &mut DeviceContainer,
    ) -> Self {
        let set_options = text
            .text
            .chars()
            .map(|c| text_pipeline.get_or_create_set(device_container, c, &text.font))
            .collect::<Vec<_>>();

        let mut x_offset = 0.;
        let mut buffers_vec = Vec::new();
        let mut sets = Vec::new();
        for (set_option, metrics) in set_options {
            sets.push(match set_option {
                Some(set) => set,
                None => {
                    x_offset += metrics.advance_width;
                    continue;
                }
            });
            buffers_vec.push(create_buffers(device_container, metrics, x_offset as i32));
            x_offset += metrics.advance_width;
        }
        Self {
            text,
            sets,
            buffers_vec,
        }
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

        let sets_and_buffers = self
            .sets
            .clone()
            .iter()
            .zip(self.buffers_vec.iter())
            .map(|(set, buffers)| (set.clone(), buffers.clone()))
            .collect::<Vec<_>>();

        text_pipeline.draw(
            device_container,
            text_fs::ty::Constants {
                resolution: device_container.resolution(),
                position: self.text.position.as_ref().clone(),
                color: self.text.color.as_ref().clone(),
                cameraPos,
            },
            sets_and_buffers,
        );
    }
}

fn create_buffers(
    device_container: &mut DeviceContainer,
    metrics: Metrics,
    x_offset: i32,
) -> BufferContainer2DUv {
    let x_min = x_offset as f32 + metrics.xmin as f32;
    let x_max = x_min + metrics.width as f32;
    let y_max = -metrics.ymin as f32;
    let y_min = y_max - metrics.height as f32;
    // let x_min = x_offset as f32 + metrics.bounds.xmin as f32;
    // let x_max = x_min + metrics.bounds.width as f32;
    // let y_max = -metrics.bounds.ymin as f32;
    // let y_min = y_max - metrics.bounds.height as f32;
    let vertices = vec![
        Vertex2DUv {
            position: [x_min, y_min],
            uvCoord: [0., 0.],
        },
        Vertex2DUv {
            position: [x_max, y_min],
            uvCoord: [1., 0.],
        },
        Vertex2DUv {
            position: [x_min, y_max],
            uvCoord: [0., 1.],
        },
        Vertex2DUv {
            position: [x_max, y_max],
            uvCoord: [1., 1.],
        },
    ];

    let indices = vec![0, 1, 2, 2, 1, 3];

    create_buffers_2d_uv(device_container, vertices, indices)
}
