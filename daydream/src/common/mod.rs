#[derive(Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rectangle {
    pub top: i32,
    pub left: i32,
    pub height: i32,
    pub width: i32,
}

impl Rectangle {
    pub fn new(top: i32, left: i32, height: i32, width: i32) -> Self {
        Rectangle {
            top,
            left,
            height,
            width,
        }
    }
}
