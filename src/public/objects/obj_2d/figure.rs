use image::ImageBuffer;
use nalgebra_glm::{UVec2, Vec2};
use vulkano::buffer::BufferContents;

use crate::Border;

use super::DEFAULT_POSITION_2D;

#[derive(Clone)]
pub struct Figure {
    pub(crate) image: FigureImage,
    pub(crate) image_changed: bool,
    pub position: Vec2,
    pub size: Vec2,
    pub border: Option<Border>,
}

impl Figure {
    pub fn new_with_image(image: FigureImage) -> Self {
        let size = Vec2::new(image.size.x as f32, image.size.y as f32);
        Self {
            image,
            image_changed: true,
            position: DEFAULT_POSITION_2D,
            size,
            border: None,
        }
    }
}

#[derive(Clone)]
pub enum FigureImageFormat {
    Rgb,
    Rgba,
}

#[derive(Clone)]
pub struct FigureImage {
    pub(crate) size: UVec2,
    pub(crate) image: Vec<u8>,
    pub(crate) image_format: FigureImageFormat,
}

impl FigureImage {
    pub fn new(size: UVec2, image: Vec<u8>, image_format: FigureImageFormat) -> Self {
        Self {
            size,
            image,
            image_format,
        }
    }

    pub fn new_default() -> Self {
        Self::new(UVec2::new(100, 100), Vec::new(), FigureImageFormat::Rgba)
    }

    pub fn new_with_size(size: UVec2) -> Self {
        let mut imgbuf = ImageBuffer::new(size.x, size.y);

        for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = image::Rgba::<u8>([0, 0, 0, 255]);
        }

        println!("{:?}", imgbuf.as_bytes().to_vec());

        Self {
            size,
            image: imgbuf.as_bytes().to_vec(),
            image_format: FigureImageFormat::Rgba,
        }
    }
}
