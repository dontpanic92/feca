use xcdt::XcDataType;

use crate::dom::{
    core::{element::ElementProps, node::NodeProps},
    Node,
};

use super::{
    html_element::{CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps},
    Paragraph,
};

xcdt::declare_xcdt!(
    CoreParagraph,
    ParagraphProps,
    CoreHtmlElement,
    CoreHtmlElementBase
);

pub struct ParagraphProps {}

impl<T: XcDataType> Paragraph for CoreParagraphBase<T> {}

pub fn new_core_paragraph(children: Vec<Box<dyn Node>>) -> Box<CoreParagraph> {
    Box::new(
        CoreParagraph::builder()
            .with(NodeProps::new(2, children))
            .with(ElementProps::new(None))
            .with(HtmlElementProps::new(None))
            .with(ParagraphProps {})
            .build(),
    )
}

/*impl XcParagraph {
    pub fn new(children: Box<dyn Node>) -> Self {
        Self::new_with(XcNode::new(XcNodeProperties{}, XcElement::new()))
    }
}*/
