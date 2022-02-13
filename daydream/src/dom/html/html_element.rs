use xcdt::XcDataType;

use super::HtmlElement;
use crate::dom::core::element::{CoreElement, CoreElementBase};

xcdt::declare_xcdt!(
    CoreHtmlElement,
    HtmlElementProps,
    CoreElement,
    CoreElementBase
);

pub struct HtmlElementProps {
    title: Option<String>,
}

impl HtmlElementProps {
    pub fn new(title: Option<String>) -> Self {
        Self { title }
    }
}

impl<T: XcDataType> HtmlElement for CoreHtmlElementBase<T> {
    fn title(&self) -> Option<&str> {
        self.ext().ext().ext().properties.title.as_deref()
    }
}

/*impl<T: XcDataType> CoreHtmlElementBase<T> {
    pub fn test(&self) {
        println!("{}", 2);
    }
}*/
