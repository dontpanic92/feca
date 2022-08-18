pub mod block;
pub mod parser;
pub mod selector;

use crate::common::Color;

#[derive(Clone, Default, Debug)]
pub struct Style {
    pub text_color: Option<Color>,
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<String>,
    pub font_weight: Option<String>,
    pub text_decoration_line: Option<TextDecorationLine>,
    pub background_color: Option<Color>,
    pub display: Display,
    pub justify_content: Option<JustifyContent>,
}

impl Style {
    pub fn html_default() -> Self {
        Self {
            text_color: Some(Color::BLACK),
            font_family: Some("Microsoft YaHei".to_string()),
            font_style: Some(FontStyle::Normal),
            font_size: Some("12px".to_string()),

            ..Default::default()
        }
    }

    pub fn from_key_value_list(list: &[(&str, &str)]) -> Self {
        let mut style = Self {
            ..Default::default()
        };

        for (key, value) in list {
            match *key {
                "display" => {
                    style.display = Display::try_from(*value).unwrap_or_default();
                }
                "justify-content" => {
                    style.justify_content =
                        Some(JustifyContent::try_from(*value).unwrap_or_default());
                }
                _ => {}
            }
        }

        style
    }

    pub fn merge(child: &Style, parent: &Style) -> Self {
        let mut ret = Style::default();

        macro_rules! merge_style {
            ($prop: ident) => {
                ret.$prop = child.$prop.clone().or_else(|| parent.$prop.clone())
            };
        }

        macro_rules! merge_style_inherit {
            ($prop: ident, $prop_ty: ty) => {
                ret.$prop = child.$prop.clone().map(|p| match p {
                    None | Some(<$prop_ty>::Inherit) => parent.$prop.clone(),
                    _ => p,
                })
            };
        }

        macro_rules! merge_style_inherit2 {
            ($prop: ident, $prop_ty: ty) => {
                ret.$prop = match child.$prop {
                    <$prop_ty>::Inherit => parent.$prop.clone(),
                    _ => child.$prop.clone(),
                }
            };
        }

        merge_style!(text_color);
        merge_style!(font_family);
        merge_style!(font_style);
        merge_style!(font_size);
        merge_style!(font_weight);
        merge_style!(text_decoration_line);
        merge_style!(background_color);
        merge_style_inherit2!(display, Display);
        merge_style!(justify_content);

        ret
    }
}

/*
macro_rules! style_context_declare {
    ($car:ident: $car_ty: ty $(, $cdr:ident: $cdr_ty: ty)* $(,)*) => {

        #[derive(Clone, Debug)]
        pub struct StyleContext {
            pub $car: Option<$car_ty>,
            $(pub $cdr : Option<$cdr_ty>,)*
        }

        impl StyleContext {
            pub fn from_style(style: &Style) -> Self {
                Self {
                    $car: style.$car.clone(),
                    $($cdr: style.$cdr.clone(),)*
                }
            }

            pub fn merge(style_context: &StyleContext, style: &Style) -> StyleContext {
                Self {
                    $car: style.$car.clone().or_else(|| style_context.$car.clone()),
                    $($cdr: style.$cdr.clone().or_else(|| style_context.$cdr.clone()),)*
                }
            }
        }
    };
}

style_context_declare!(
    text_color: Color,
    font_family: String,
    font_style: FontStyle,
    font_size: String,
    font_weight: String,
    text_decoration_line: TextDecorationLine,
);
*/

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextDecorationLine {
    Overline,
    LineThrough,
    Underline,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Display {
    Block,
    Inline,
    Inherit,
    Flex,
    None,

    // Internal use
    FelisText,
}

impl From<&str> for Display {
    fn from(value: &str) -> Self {
        match value {
            "block" => Display::Block,
            "inline" => Display::Inline,
            "flex" => Display::Flex,
            _ => Display::Block,
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::Inherit
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> Self {
        Self::FlexStart
    }
}

impl From<&str> for JustifyContent {
    fn from(value: &str) -> Self {
        println!("{value}");
        match value {
            "flex-start" => JustifyContent::FlexStart,
            "flex-end" => JustifyContent::FlexEnd,
            "start" => JustifyContent::Start,
            "end" => JustifyContent::End,
            "center" => JustifyContent::Center,
            "space-between" => JustifyContent::SpaceBetween,
            "space-around" => JustifyContent::SpaceAround,
            "space-evenly" => JustifyContent::SpaceEvenly,
            _ => JustifyContent::FlexStart,
        }
    }
}
