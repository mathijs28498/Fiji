use nalgebra_glm::{Vec3, Vec4};

use fiji::{
    input::input_enums::KeyCode,
    objects::{background::Background, obj_3d::block::Block},
    rendering::context::Context,
};

fn main() {
    let mut context = Context::new(1280, 720);

    let mut block = Block::new(
        Vec4::new(1., 1., 0., 1.),
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 1., 1.),
    );

    context.event_loop().run(move |input, event_handler| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        if input.key_held(&KeyCode::A) {
            block.position.x += 0.01;
        }
        if input.key_held(&KeyCode::D) {
            block.position.x -= 0.01;
        }
        if input.key_held(&KeyCode::W) {
            block.position.z += 0.01;
        }
        if input.key_held(&KeyCode::S) {
            block.position.z -= 0.01;
        }

        context.draw_background(Background::new(Vec3::new(0., 1., 1.)));
        context.draw_block(block.clone());

        context.render();
    })
}
