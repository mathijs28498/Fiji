use fiji::{Context, Figure, FigureImage, KeyCode};
use nalgebra_glm::UVec2;

pub fn main() {
    let mut context = Context::new(1280, 720);

    let figure = Figure::new_with_image(FigureImage::new_with_size(UVec2::new(10, 10)));

    context.event_loop().run(move |input, event_handler| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        context.figure(figure.clone());

        context.render(event_handler);
    })
}
