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

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub fn to_color_f(&self) -> ColorF {
        ColorF {
            r: self.r as f64 / 1.,
            g: self.g as f64 / 1.,
            b: self.b as f64 / 1.,
            a: self.a as f64 / 1.,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ColorF {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
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
