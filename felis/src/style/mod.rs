pub mod block;
pub mod parser;
pub mod selector;

use felis_macros::{FelisDefault, FelisStyle};
use strum::EnumString;

use crate::common::Color;

#[derive(Clone, Default, Debug, FelisStyle)]
pub struct Style {
    #[prop]
    pub color: Option<Color>,

    #[prop]
    pub font_family: Option<String>,

    #[prop]
    pub font_style: Option<FontStyle>,

    #[prop]
    pub font_size: Option<String>,

    #[prop]
    pub font_weight: Option<String>,

    #[prop]
    pub text_decoration_line: Option<TextDecorationLine>,

    #[prop]
    pub background_color: Option<Color>,

    #[prop]
    pub display: Option<Display>,

    #[prop]
    pub justify_content: Option<JustifyContent>,
}

impl Style {
    pub fn html_default() -> Self {
        Self {
            color: Some(Color::BLACK),
            font_family: Some("Microsoft YaHei".to_string()),
            font_style: Some(FontStyle::Normal),
            font_size: Some("12px".to_string()),

            ..Default::default()
        }
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
