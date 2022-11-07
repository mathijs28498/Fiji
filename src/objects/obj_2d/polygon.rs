use nalgebra_glm::{dot, Vec2, Vec4};

use crate::{
    objects::{help_functions::create_buffers_2d, Border},
    rendering::{
        data_types::{BufferContainer2D, Vertex2D},
        device_container::DeviceContainer,
        render_passes::poly_render_pass::PolyRenderPass,
    },
};

#[derive(Clone)]
pub struct Polygon {
    pub color: Vec4,
    pub points: Vec<Vec2>,
    pub border: Option<Border>,
    buffers: Option<BufferContainer2D>,
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
            self.buffers = Some(self.create_buffers(device_container));
        }

        render_pass.draw(
            device_container,
            self.buffers.as_ref().unwrap(),
            PolyRenderPass::create_push_constants(
                self.color.clone(),
                Vec2::new(0., 0.),
                Vec2::new(1., 1.),
                self.border.clone(),
            ),
        );
    }

    fn create_buffers(&self, device_container: &mut DeviceContainer) -> BufferContainer2D {
        // TODO: Use iter().map() on self.points
        let mut vertices = Vec::new();
        for p in &self.points {
            vertices.push(Vertex2D {
                position: [p.x, p.y],
            })
        }
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

        create_buffers_2d(device_container, vertices, indices)
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
