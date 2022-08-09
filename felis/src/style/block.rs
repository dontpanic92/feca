use super::{selector::TrivalSelector, Style};

#[derive(Debug)]
pub struct StyleBlock {
    pub selectors: Vec<TrivalSelector>,
    pub style: Style,
}
