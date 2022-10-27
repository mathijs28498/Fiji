use nalgebra_glm::{Vec3, Vec4};

use fiji::{
    input::input_enums::KeyCode,
    objects::{background::Background, obj_3d::block::Block},
    rendering::context::Context,
};

fn main() {
    let mut context = Context::new(1280, 720);

    let block = Block::new(
        Vec4::new(1., 0., 0., 1.),
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 1., 1.),
    );

    context.event_loop().run(move |input, event_handler| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        context.draw_background(Background::new(Vec3::new(0., 1., 1.)));
        context.draw_block(block.clone());

        context.render();
    })
}
