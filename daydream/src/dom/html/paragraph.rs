use xcdt::XcDataType;

use crate::{
    dom::{
        core::{
            element::ElementProps,
            node::{CoreNode, NodeProps},
        },
        Node,
    },
    layout::{flow::FlowLayout, Test},
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

impl ParagraphProps {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: XcDataType> Paragraph for CoreParagraphBase<T> {}

/*
pub trait Test2 {
    fn test(&self);
}

impl Test for dyn Test2 {
    fn test(&self) {
        <Self as Test2>::test(self)
    }
}

impl Test2 for CoreParagraph {
    fn test(&self) {
        println!("in paragraph test");
    }
}*/

// impl<T: XcDataType> ReuseTestImpl for CoreParagraphBase<T> {}

pub trait ImplTestViaCoreParagraph {}
impl<T: ImplTestViaCoreParagraph> Test for T {
    fn test(&self) {
        println!("in paragraph test");
    }
}

pub fn new_core_paragraph(children: Vec<Box<dyn Node>>) -> Box<CoreParagraph> {
    Box::new(
        CoreParagraph::builder()
            .with(NodeProps::new(2, children))
            .with(ElementProps::new(None))
            .with(HtmlElementProps::new(None))
            .with(ParagraphProps::new())
            .build(),
    )
}
