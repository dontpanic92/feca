use crate::dom::core::{Nil, element::CoreElementBase, XcDataType};

use super::HtmlElement;

crate::declare_xcdt!(CoreHtmlElement, HtmlElementProps, CoreElementBase);

pub struct HtmlElementProps {
    title: String,
}

impl<T: XcDataType> HtmlElement for CoreHtmlElementBase<T> {
    fn title(&self) -> String {
        self.as_CoreHtmlElementBase().properties.title.clone()
    }
}


impl<T: XcDataType> CoreHtmlElementBase<T> {
    pub fn test(&self) {
        println!("{}", 2);
    }
}
