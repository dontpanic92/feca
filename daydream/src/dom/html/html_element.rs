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
    title: String,
}

impl HtmlElementProps {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

impl<T: XcDataType> HtmlElement for CoreHtmlElementBase<T> {
    fn title(&self) -> String {
        self.ext().ext().ext().properties.title.clone()
    }
}

/*impl<T: XcDataType> CoreHtmlElementBase<T> {
    pub fn test(&self) {
        println!("{}", 2);
    }
}*/
