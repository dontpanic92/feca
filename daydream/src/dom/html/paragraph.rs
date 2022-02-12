use super::{
    html_element::{CoreHtmlElement, CoreHtmlElementBase},
    Paragraph,
};
use xcdt::XcDataType;

xcdt::declare_xcdt!(
    CoreParagraph,
    ParagraphProps,
    CoreHtmlElement,
    CoreHtmlElementBase
);

pub struct ParagraphProps {}

impl<T: XcDataType> Paragraph for CoreParagraphBase<T> {}

// pub fn new_core_paragraph(children: Vec<Box<dyn Node>>) -> CoreParagraph {}

/*impl XcParagraph {
    pub fn new(children: Box<dyn Node>) -> Self {
        Self::new_with(XcNode::new(XcNodeProperties{}, XcElement::new()))
    }
}*/
