pub mod square;
pub mod background;

use self::{square::Square, background::Background};

pub enum DrawObject {
    SquareObject(Square),
    BackgroundObject(Background),
}

// pub trait DrawObject {
//     fn get_buffers(
//         &mut self,
//         queue: &Arc<Queue>,
//     ) -> (Arc<ImmutableBuffer<[Vertex]>>, Arc<ImmutableBuffer<[u32]>>);

//     fn render_pass_type(&self) -> RenderPassType;
// }


// impl Square {
//     pub fn new(position: glm::Vec2, size: glm::Vec2, color: glm::Vec3) -> Self {
//         Self {
//             position,
//             size,
//             color,
//             vertex_buffer: None,
//             index_buffer: None,
//             render_pass_type: RenderPassType::Poly,
//         }
//     }
// }

// impl DrawObject for Square {

//     fn render_pass_type(&self) -> RenderPassType {
//         self.render_pass_type.clone()
//     }
// }

// pub struct ClearBackground {

// }

// impl DrawObject for ClearBackground {

// }
