use crate::common::Rectangle;

pub struct CssBoxLayoutModel {
    boundary: Rectangle,
}

impl CssBoxLayoutModel {
    pub fn new() -> Self {
        Self {
            boundary: Rectangle {
                top: 0,
                left: 0,
                width: 60,
                height: 60,
            },
        }
    }

    pub fn padding_boundary(&self) -> Rectangle {
        self.boundary
    }

    pub fn content_boundary(&self) -> Rectangle {
        self.boundary
    }
}
