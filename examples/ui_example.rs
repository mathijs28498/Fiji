use fiji::{
    input::input_enums::KeyCode,
    public::{
        context::Context,
        objects::{
            background::Background,
            obj_2d::{rect::Rect, text::Text},
        },
    },
};
use nalgebra_glm::{Vec2, Vec3, Vec4};

pub fn main() {
    let mut context = Context::new(1280, 720);
    context.background(Background::new_with_color(Vec3::new(0.3, 0.1, 0.1)));

    let r = Rect::new_default()
        .with_color(Vec4::new(0.7, 0.3, 0.7, 1.))
        .with_position(Vec2::new(400., 400.))
        .with_size(Vec2::new(100., 200.));

    let r_2 = r
        .clone()
        .with_color(Vec4::new(0., 1., 0., 0.3))
        .with_position(r.position + Vec2::new(200., 0.));

    context.event_loop().run(move |input, event_handler| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        context.rect(r.clone());
        context.text(Text::new_with_text("This is a test"));
        context.ui_text(
            Text::new_with_text("This is another test!!!")
                .with_position(Vec2::new(400., 300.))
                .with_color(Vec4::new(0.2, 0.7, 0.9, 1.)),
        );
        context.ui_rect(r_2.clone());

        context.render(event_handler);
    });
}
