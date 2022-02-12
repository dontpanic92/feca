use super::HtmlElement;
use crate::dom::core::element::{CoreElement, CoreElementBase};
use xcdt::XcDataType;

xcdt::declare_xcdt!(
    CoreHtmlElement,
    HtmlElementProps,
    CoreElement,
    CoreElementBase
);

pub struct HtmlElementProps {
    title: String,
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
