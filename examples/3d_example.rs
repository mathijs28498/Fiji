use std::f32::consts::{FRAC_PI_2, PI};

use nalgebra_glm::{Vec2, Vec3, Vec4};

use fiji::{
    input::input_enums::KeyCode,
    public::{
        context::Context,
        objects::{background::Background, obj_3d::block::Block},
    },
};

fn main() {
    let mut context = Context::new(1280, 720);

    let mut block_0 = Block::new_default()
        .with_color(Vec4::new(0.9, 0.57, 0.28, 1.))
        .with_size(Vec3::new(1., 10., 0.1))
        .with_rotation(Vec3::new(0.3, 0.6, -0.2));

    let mut block_3 = Block::new_default()
        .with_color(Vec4::new(0.28, 0.57, 0.9, 1.))
        .with_position(Vec3::new(0., 0., -3.));

    let mut block_5 = Block::new_default()
        .with_color(Vec4::new(0.57, 0.28, 0.9, 1.))
        .with_position(Vec3::new(2., 0., -5.))
        .with_size(Vec3::new(2., 2., 1.))
        .with_rotation(Vec3::new(1., -0., 0.5));

    let mut wall = Block::new_default()
        .with_color(Vec4::new(0.57, 0.28, 0.9, 1.))
        .with_position(Vec3::new(0., 40., -10.))
        .with_size(Vec3::new(100., 100., 1.));

    let mut ground = Block::new_default()
        .with_color(Vec4::new(0.28, 0.9, 0.57, 1.))
        .with_position(Vec3::new(0., -10., 35.))
        .with_size(Vec3::new(100., 0.1, 100.));

    let mut rotate_camera = false;
    let mut last_phi = -FRAC_PI_2;
    let mut last_theta = 0.;
    let sensitivity = 0.01;
    let speed = 0.04;

    context.event_loop().run(move |input, event_handler| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        let mut move_dir = Vec3::new(0., 0., 0.);
        let up = Vec3::new(0., 1., 0.);
        let mut forward = context.camera_3d.dir.clone();
        forward.y = 0.;
        forward = forward.normalize();
        let right = forward.cross(&up).normalize();

        if input.key_held(&KeyCode::A) {
            move_dir -= right;
        }
        if input.key_held(&KeyCode::D) {
            move_dir += right;
        }
        if input.key_held(&KeyCode::W) {
            move_dir += forward;
        }
        if input.key_held(&KeyCode::S) {
            move_dir -= forward;
        }
        if input.key_held(&KeyCode::Z) {
            move_dir.y += 1.;
        }
        if input.key_held(&KeyCode::X) {
            move_dir.y -= 1.;
        }
        if input.key_pressed(&KeyCode::C) {
            rotate_camera = !rotate_camera;
        }

        if rotate_camera {
            let md = input.mouse_delta();
            last_phi = (last_phi + sensitivity * md.x) % (2. * PI);

            last_theta = last_theta + sensitivity * md.y;
            if last_theta < -FRAC_PI_2 {
                last_theta = -FRAC_PI_2 + 0.00001;
            } else if last_theta > FRAC_PI_2 {
                last_theta = FRAC_PI_2 - 0.00001;
            }

            context.camera_3d.dir = Vec3::new(
                last_theta.cos() * last_phi.cos(),
                last_theta.sin(),
                last_theta.cos() * last_phi.sin(),
            );
        }

        if move_dir.norm_squared() > 0.1 {
            context.camera_3d.position += move_dir.normalize() * speed;
        }

        block_0.rotation.x += 0.01;

        context.background(Background::new_with_color(Vec3::new(0.07, 0.51, 0.6)));
        context.block(block_0.clone());
        context.block(block_5.clone());
        context.block(block_3.clone());
        context.block(wall.clone());
        context.block(ground.clone());

        context.render();
    })
}
