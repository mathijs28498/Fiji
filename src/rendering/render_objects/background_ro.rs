use crate::public::objects::background::Background;

pub(crate) struct BackgroundRenderObject {
    pub(crate) background: Background,
}

impl BackgroundRenderObject {
    pub(crate) fn new(background: Background) -> Self {
        Self { background }
    }
}
