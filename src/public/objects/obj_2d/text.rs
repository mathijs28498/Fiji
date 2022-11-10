#[derive(Clone, Debug)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new_with_text(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}
