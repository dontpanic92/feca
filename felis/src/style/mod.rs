use crate::common::Color;

#[derive(Clone, Default, Debug)]
pub struct Style {
    pub text_color: Option<Color>,
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<String>,
    pub font_weight: Option<String>,
    pub text_decoration_line: Option<TextDecorationLine>,
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
}

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
