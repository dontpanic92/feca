use super::HtmlElement;
use crate::dom::core::element::CoreElementBase;
use xcdt::{Nil, XcDataType};

xcdt::declare_xcdt!(CoreHtmlElement, HtmlElementProps, CoreElementBase);

pub struct HtmlElementProps {
    title: String,
}

impl<T: XcDataType> HtmlElement for CoreHtmlElementBase<T> {
    fn title(&self) -> String {
        self.as_CoreHtmlElementBase().properties.title.clone()
    }
}

/*impl<T: XcDataType> CoreHtmlElementBase<T> {
    pub fn test(&self) {
        println!("{}", 2);
    }
}*/
