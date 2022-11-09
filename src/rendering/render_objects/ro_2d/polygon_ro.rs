use nalgebra_glm::{dot, Vec2};

use crate::{
    public::objects::{camera::camera_2d::Camera2D, obj_2d::polygon::Polygon},
    rendering::{
        render_containers::device_container::DeviceContainer,
        render_objects::shared::{create_buffers_2d, BufferContainer2D, Vertex2D},
        render_passes::render_passes_2d::poly_render_pass::PolyRenderPass,
    },
};

#[derive(Clone)]
pub(crate) struct PolygonRenderObject {
    pub(crate) polygon: Polygon,
    buffers: BufferContainer2D,
}

impl PolygonRenderObject {
    pub(crate) fn new(polygon: Polygon, device_container: &mut DeviceContainer) -> Self {
        let buffers = Self::create_buffers(&polygon, device_container);
        Self { polygon, buffers }
    }

    pub(crate) fn draw(
        &mut self,
        render_pass: &mut PolyRenderPass,
        device_container: &mut DeviceContainer,
        camera_2d: Option<&Camera2D>,
    ) {
        render_pass.draw(
            device_container,
            &self.buffers,
            PolyRenderPass::create_push_constants(
                self.polygon.color.clone(),
                Vec2::new(0., 0.),
                Vec2::new(1., 1.),
                self.polygon.border.clone(),
                camera_2d,
            ),
        );
    }

    fn create_buffers(
        polygon: &Polygon,
        device_container: &mut DeviceContainer,
    ) -> BufferContainer2D {
        // TODO: Use iter().map() on polygon.points
        let mut vertices = Vec::new();
        for p in &polygon.points {
            vertices.push(Vertex2D {
                position: [p.x, p.y],
            })
        }
        let mut indices = vec![0, 1, 2];

        for (i, p) in polygon.points.iter().enumerate().skip(3) {
            let index_0;
            let index_1;
            {
                let last_triangle = &indices[indices.len() - 3..indices.len()];
                let p0 = polygon.points[last_triangle[0] as usize].clone();
                let p1 = polygon.points[last_triangle[1] as usize].clone();
                let p2 = polygon.points[last_triangle[2] as usize].clone();

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
