use std::rc::Rc;

use crate::{layout::Layoutable, rendering::Renderable};

pub mod core;
pub mod html;

pub trait Node {
    fn children(&self) -> &[Rc<dyn Node>];
    fn inner_html(&self) -> String;
    fn get_element_by_id(&self, element_id: &str) -> Option<Rc<dyn Node>>;

    fn as_layoutable(&self) -> &dyn Layoutable;
    fn as_renderable(&self) -> &dyn Renderable;
    fn as_internal(&self) -> &dyn NodeInternal;
}

pub trait Element: Node {
    fn id(&self) -> Option<&str>;
}

pub trait CharacterData: Node {
    fn text(&self) -> &str;
}

pub trait Text: CharacterData {
    fn split_text(&self, offset: usize) -> Box<dyn Text>;
}

pub trait NodeInternal {
    fn collect_outer_html(&self, frag_list: &mut Vec<String>);
}
