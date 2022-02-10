use xcdt::{Nil, XcDataType};

use super::{html_element::CoreHtmlElementBase, Paragraph};

xcdt::declare_xcdt!(CoreParagraph, ParagraphProps, CoreHtmlElementBase);

pub struct ParagraphProps {}

impl<T: XcDataType> Paragraph for CoreParagraphBase<T> {}

/*impl XcParagraph {
    pub fn new(children: Box<dyn Node>) -> Self {
        Self::new_with(XcNode::new(XcNodeProperties{}, XcElement::new()))
    }
}*/
