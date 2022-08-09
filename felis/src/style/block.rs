use super::{selector::TrivalSelector, Style};

pub struct StyleBlock {
    pub selectors: Vec<TrivalSelector>,
    pub style: Style,
}