use nalgebra_glm::{Vec3, Vec4};

use fiji::{
    input::input_enums::KeyCode,
    objects::{background::Background, obj_3d::block::Block},
    rendering::context::Context,
};

fn main() {
    let mut context = Context::new(1280, 720);

    let mut block = Block::new(
        Vec4::new(0.9, 0.57, 0.28, 1.),
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 1., 1.),
    );

    context.event_loop().run(move |input, event_handler| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        if input.key_held(&KeyCode::A) {
            context.camera_3d.position.x -= 0.04;
        }
        if input.key_held(&KeyCode::D) {
            context.camera_3d.position.x += 0.04;
        }
        if input.key_held(&KeyCode::W) {
            context.camera_3d.position.z -= 0.04;
        }
        if input.key_held(&KeyCode::S) {
            context.camera_3d.position.z += 0.04;
        }
        if input.key_held(&KeyCode::Z) {
            context.camera_3d.position.y += 0.04;
        }
        if input.key_held(&KeyCode::X) {
            context.camera_3d.position.y -= 0.04;
        }

        context.draw_background(Background::new(Vec3::new(0.07, 0.51, 0.6)));
        context.draw_block(block.clone());

        context.render();
    })
}
