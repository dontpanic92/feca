use super::{selector::SelectorCombinator, Style};

#[derive(Debug)]
pub struct StyleBlock {
    pub selectors: Vec<SelectorCombinator>,
    pub style: Style,
}
