use std::str::FromStr;

use crate::style::parser::Property;

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

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::BLACK)
    }
}

impl From<&Property> for Color {
    fn from(_: &Property) -> Self {
        Self::BLACK
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

    pub fn new_ltrb(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            top,
            left,
            height: bottom - top,
            width: right - left,
        }
    }

    pub fn top(&self) -> i32 {
        self.top
    }

    pub fn left(&self) -> i32 {
        self.left
    }

    pub fn right(&self) -> i32 {
        self.left + self.width
    }

    pub fn bottom(&self) -> i32 {
        self.top + self.height
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn point_in(&self, x: f64, y: f64) -> bool {
        x >= self.left as f64
            && x <= self.right() as f64
            && y >= self.top as f64
            && y <= self.bottom() as f64
    }
}
