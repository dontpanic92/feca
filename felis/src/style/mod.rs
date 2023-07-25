pub mod at_rules;
pub mod block;
pub mod parser;
pub mod selector;

use std::str::FromStr;

use felis_macros::{FelisDefault, FelisStyle};
use strum::EnumString;

use crate::common::Color;

use self::parser::{Property, PropertyValue};

#[derive(Clone, Default, Debug, FelisStyle)]
pub struct Style {
    #[prop]
    pub color: Option<Color>,

    #[prop]
    pub font_family: Option<String>,

    #[prop]
    pub font_style: Option<FontStyle>,

    #[prop]
    pub font_size: Option<Length>,

    #[prop]
    pub font_weight: Option<FontWeight>,

    #[prop]
    pub text_decoration_line: Option<TextDecorationLine>,

    #[prop]
    pub background_color: Option<Color>,

    #[prop]
    pub display: Option<Display>,

    #[prop]
    pub justify_content: Option<JustifyContent>,

    #[prop]
    pub border_top_width: Option<Length>,

    #[prop]
    pub border_top_color: Option<Color>,

    #[prop]
    pub border_left_width: Option<Length>,

    #[prop]
    pub border_left_color: Option<Color>,

    #[prop]
    pub border_right_width: Option<Length>,

    #[prop]
    pub border_right_color: Option<Color>,

    #[prop]
    pub border_bottom_width: Option<Length>,

    #[prop]
    pub border_bottom_color: Option<Color>,
}

impl Style {
    pub fn html_default() -> Self {
        Self {
            color: Some(Color::BLACK),
            font_family: Some("Microsoft YaHei".to_string()),
            font_style: Some(FontStyle::Normal),
            font_size: Some(Length::new("12px")),

            ..Default::default()
        }
    }
}

impl From<&Property> for String {
    fn from(p: &Property) -> Self {
        p.value
            .iter()
            .map(|v| {
                v.iter()
                    .map(|pv| match pv {
                        PropertyValue::String(s) => s,
                        PropertyValue::FunctionCall => "",
                    })
                    .collect::<Vec<&str>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join(",")
    }
}

#[derive(Clone, Copy, PartialEq, Debug, EnumString, FelisDefault)]
pub enum FontStyle {
    #[strum(serialize = "normal")]
    #[default_item]
    Normal,
    #[strum(serialize = "italic")]
    Italic,
    #[strum(serialize = "oblique")]
    Oblique,
}

impl From<&Property> for FontStyle {
    fn from(p: &Property) -> Self {
        let s: String = p.into();
        s.parse().unwrap_or(FontStyle::Normal)
    }
}

#[derive(Clone, Copy, PartialEq, Debug, EnumString, FelisDefault)]
pub enum TextDecorationLine {
    #[strum(serialize = "none")]
    #[default_item]
    None,
    #[strum(serialize = "overline")]
    Overline,
    #[strum(serialize = "line-through")]
    LineThrough,
    #[strum(serialize = "underline")]
    Underline,
}

impl From<&Property> for TextDecorationLine {
    fn from(p: &Property) -> Self {
        let s: String = p.into();
        s.parse().unwrap_or(Self::None)
    }
}

#[derive(Copy, Clone, PartialEq, Debug, EnumString, FelisDefault)]
pub enum Display {
    #[strum(serialize = "block")]
    Block,
    #[strum(serialize = "inline")]
    Inline,
    #[strum(serialize = "inherit")]
    #[default_item]
    Inherit,
    #[strum(serialize = "flex")]
    Flex,
    #[strum(serialize = "none")]
    None,

    // Internal use
    #[strum(disabled)]
    FelisText,
}

impl From<&Property> for Display {
    fn from(p: &Property) -> Self {
        let s: String = p.into();
        s.parse().unwrap_or(Self::Inherit)
    }
}

#[derive(Copy, Clone, PartialEq, Debug, EnumString, FelisDefault)]
pub enum JustifyContent {
    #[strum(serialize = "flex-start")]
    #[default_item]
    FlexStart,
    #[strum(serialize = "flex-end")]
    FlexEnd,
    #[strum(serialize = "start")]
    Start,
    #[strum(serialize = "end")]
    End,
    #[strum(serialize = "center")]
    Center,
    #[strum(serialize = "space-between")]
    SpaceBetween,
    #[strum(serialize = "space-around")]
    SpaceAround,
    #[strum(serialize = "space-evenly")]
    SpaceEvenly,
}

impl From<&Property> for JustifyContent {
    fn from(p: &Property) -> Self {
        let s: String = p.into();
        s.parse().unwrap_or(Self::FlexStart)
    }
}

#[derive(Clone, Debug)]
pub struct Length(String);

impl Length {
    pub fn new(desc: &str) -> Self {
        Self(desc.to_string())
    }

    pub fn desc(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&Property> for Length {
    fn from(p: &Property) -> Self {
        Self { 0: p.into() }
    }
}

impl FromStr for Length {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl Default for Length {
    fn default() -> Self {
        Self::new("12px")
    }
}

#[derive(Clone, Debug)]
pub struct FontWeight(String);

impl FontWeight {
    pub fn new(desc: &str) -> Self {
        Self(desc.to_string())
    }

    pub fn desc(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&Property> for FontWeight {
    fn from(p: &Property) -> Self {
        Self { 0: p.into() }
    }
}

impl FromStr for FontWeight {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { 0: s.into() })
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::new("400")
    }
}
