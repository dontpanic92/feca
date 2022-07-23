use std::{collections::HashMap, rc::Rc};

use crate::{
    common::Rectangle,
    dom::{Node, NodeInternal},
    layout::{flow::FlowLayout, Layoutable},
    rendering::Renderable,
    style::Style,
};
use xcdt::{Object, ObjectBase, XcDataType};

xcdt::declare_xcdt!(CoreNode, NodeProps, Object, ObjectBase);

pub enum NodeType {
    ElementNode = 1,
    AttributeNode = 2,
    TextNode = 3,
    CDataSectionNode = 4,
    EntityReferenceNode = 5, // Legacy
    EntityNode = 6,          // Legacy
    ProcessingInstructionNode = 7,
    CommentNode = 8,
    DocumentNode = 9,
    DocumentTypeNode = 10,
    DocumentFragmentNode = 11,
    NotationNode = 12, // Legacy
}

pub struct NodeProps {
    node_type: NodeType,
    children: Vec<Rc<dyn Node>>,
}

impl NodeProps {
    pub fn new(node_type: NodeType, children: Vec<Rc<dyn Node>>) -> Self {
        Self {
            node_type,
            children,
        }
    }

    pub fn children(&self) -> &[Rc<dyn Node>] {
        &self.children
    }
}

impl<T: 'static + XcDataType> Node for CoreNodeBase<T> {
    fn children(&self) -> &[Rc<dyn Node>] {
        self.NodeProps().children.as_ref()
    }

    fn inner_html(&self) -> String {
        let mut frag_list = vec![];
        for c in self.children() {
            c.as_internal().collect_outer_html(&mut frag_list);
        }

        frag_list.join("\n")
    }

    fn as_layoutable(&self) -> &dyn Layoutable {
        self as &dyn Layoutable
    }

    fn as_renderable(&self) -> &dyn Renderable {
        self as &dyn Renderable
    }

    fn as_internal(&self) -> &dyn NodeInternal {
        self as &dyn NodeInternal
    }

    fn get_element_by_id(&self, element_id: &str) -> Option<Rc<dyn Node>> {
        todo!()
    }
}

impl<T: 'static + XcDataType> NodeInternal for CoreNodeBase<T> {
    default fn collect_outer_html(&self, _: &mut Vec<String>) {}
}

impl<T: 'static + XcDataType> Layoutable for CoreNodeBase<T> {
    default fn layout(
        &self,
        _pango_context: &pango::Context,
        _style_computed: &Style,
        _content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        Rectangle::new(0, 0, 0, 0)
    }

    default fn display(&self) -> crate::style::Display {
        crate::style::Display::Block
    }
}

impl<T: 'static + XcDataType> Renderable for CoreNodeBase<T> {
    default fn paint(
        &self,
        _renderer: &crate::rendering::cairo::CairoRenderer,
        _style_computed: &Style,
    ) {
    }
}

pub fn layout_children(
    children: &[Rc<dyn Node>],
    pango_context: &pango::Context,
    style_computed: &Style,
    content_boundary: crate::common::Rectangle,
) -> Rectangle {
    let children: Vec<&dyn Layoutable> = children.iter().map(|c| c.as_layoutable()).collect();

    FlowLayout::layout(pango_context, style_computed, content_boundary, &children)
}

pub fn paint_children(
    children: &[Rc<dyn Node>],
    renderer: &crate::rendering::cairo::CairoRenderer,
    style_computed: &Style,
) {
    children
        .iter()
        .map(|c| c.as_renderable())
        .for_each(|c| c.paint(renderer, style_computed))
}
