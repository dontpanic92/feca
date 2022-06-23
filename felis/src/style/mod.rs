use crate::common::Color;

#[derive(Clone, Default, Debug)]
pub struct Style {
    pub text_color: Option<Color>,
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<String>,
    pub text_decoration_line: Option<TextDecorationLine>,
}

impl Style {
    pub fn html_default() -> Self {
        Self {
            text_color: Some(Color::BLACK),
            font_family: Some("Microsoft YaHei".to_string()),
            font_style: Some(FontStyle::Normal),
            font_size: Some("14px".to_string()),

            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct StyleContext {
    pub text_color: Option<Color>,
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<String>,
    pub text_decoration_line: Option<TextDecorationLine>,
}

impl StyleContext {
    pub fn from_style(style: &Style) -> Self {
        Self {
            text_color: style.text_color.clone(),
            font_family: style.font_family.clone(),
            font_style: style.font_style.clone(),
            font_size: style.font_size.clone(),
            text_decoration_line: style.text_decoration_line.clone(),
        }
    }

    pub fn merge(style_context: &StyleContext, style: &Style) -> StyleContext {
        Self {
            text_color: style
                .text_color
                .clone()
                .or_else(|| style_context.text_color.clone()),
            font_family: style
                .font_family
                .clone()
                .or_else(|| style_context.font_family.clone()),
            font_style: style
                .font_style
                .clone()
                .or_else(|| style_context.font_style.clone()),
            font_size: style
                .font_size
                .clone()
                .or_else(|| style_context.font_size.clone()),
            text_decoration_line: style
                .text_decoration_line
                .clone()
                .or_else(|| style_context.text_decoration_line.clone()),
        }
    }
}

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
