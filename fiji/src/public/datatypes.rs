#[derive(Debug, Clone)]
pub enum Color {
    Rgb(f32, f32, f32),
    Rgba(f32, f32, f32, f32),
}

impl Default for Color {
    fn default() -> Self {
        Self::Rgb(1., 1., 1.)
    }
}

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        match self {
            Self::Rgb(r, g, b) => [r, g, b],
            Self::Rgba(r, g, b, _) => [r, g, b],
        }
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        match self {
            Self::Rgb(r, g, b) => [r, g, b, 1.],
            Self::Rgba(r, g, b, a) => [r, g, b, a],
        }
    }
}
