use std::sync::Arc;

use nalgebra_glm::{dot, Vec2, Vec4};
use vulkano::{
    buffer::{BufferUsage, ImmutableBuffer},
    device::Queue,
};

use crate::rendering::{
    data_types::{BufferContainer, Vertex},
    device_container::DeviceContainer,
    render_passes::{
        circle_render_pass::CircleRenderPass,
        poly_render_pass::{PolyPushConstants, PolyRenderPass},
    },
};

use super::Border;

#[derive(Clone)]
pub struct Polygon {
    pub color: Vec4,
    pub points: Vec<Vec2>,
    pub border: Option<Border>,
    buffers: Option<BufferContainer>,
}

impl Polygon {
    pub fn new(color: Vec4, points: Vec<Vec2>, border: Option<Border>) -> Self {
        Self {
            color,
            points,
            border,
            buffers: None,
        }
    }

    pub fn new_triangle(color: Vec4, points: [Vec2; 3], border: Option<Border>) -> Self {
        Self {
            color,
            points: points.into(),
            border,
            buffers: None,
        }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut PolyRenderPass,
        device_container: &mut DeviceContainer,
    ) {
        if let None = self.buffers {
            self.buffers = Some(BufferContainer {
                vertex_buffer: self.get_vertex_buffer(device_container.queue().clone()),
                index_buffer: self.get_index_buffer(device_container.queue().clone()),
            })
        }

        render_pass.draw(
            device_container,
            self.buffers.as_ref().unwrap(),
            PolyPushConstants::new(
                self.color.clone(),
                Vec2::new(0., 0.),
                Vec2::new(1., 1.),
                self.border.clone(),
            ),
        );
    }

    // TODO: Implement proper vertex buffer shit
    fn get_vertex_buffer(&self, queue: Arc<Queue>) -> Arc<ImmutableBuffer<[Vertex]>> {
        let mut vertices = Vec::new();
        for p in &self.points {
            vertices.push(Vertex {
                position: [p.x, p.y],
            })
        }

        return ImmutableBuffer::from_iter(vertices, BufferUsage::vertex_buffer(), queue)
            .unwrap()
            .0;
    }

    fn get_index_buffer(&self, queue: Arc<Queue>) -> Arc<ImmutableBuffer<[u32]>> {
        let mut indices = vec![0, 1, 2];

        for (i, p) in self.points.iter().enumerate().skip(3) {
            let index_0;
            let index_1;
            {
                let last_triangle = &indices[indices.len() - 3..indices.len()];
                let p0 = self.points[last_triangle[0] as usize].clone();
                let p1 = self.points[last_triangle[1] as usize].clone();
                let p2 = self.points[last_triangle[2] as usize].clone();

                let mut lines = Vec::new();
                if i == 3 {
                    lines.push((
                        (last_triangle[0], last_triangle[1]),
                        dist_to_line(&p0, &p1, &p),
                    ));
                }
                lines.push((
                    (last_triangle[0], last_triangle[2]),
                    dist_to_line(&p0, &p2, &p),
                ));
                lines.push((
                    (last_triangle[1], last_triangle[2]),
                    dist_to_line(&p1, &p2, &p),
                ));
                lines.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                (index_0, index_1) = lines[0].0;
            }
            indices.push(index_0);
            indices.push(index_1);
            indices.push(i as u32);
        }

        ImmutableBuffer::from_iter(indices, BufferUsage::index_buffer(), queue.clone())
            .unwrap()
            .0
    }
}

fn dist_to_line(a: &Vec2, b: &Vec2, p: &Vec2) -> f32 {
    let l2 = (a - b).norm_squared();
    if l2 == 0. {
        return (p - a).norm_squared();
    }

    let t = 0.0_f32.max(1.0_f32.min(dot(&(p - a), &(b - a)) / l2));
    let proj = a + t * (b - a);
    (p - proj).norm_squared()
}
