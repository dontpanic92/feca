use crate::{
    common::Rectangle,
    dom::Node,
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
    children: Vec<Box<dyn Node>>,
}

impl NodeProps {
    pub fn new(node_type: NodeType, children: Vec<Box<dyn Node>>) -> Self {
        Self {
            node_type,
            children,
        }
    }

    pub fn children(&self) -> &[Box<dyn Node>] {
        &self.children
    }
}

impl<T: 'static + XcDataType> Node for CoreNodeBase<T> {
    fn children(&self) -> &[Box<dyn Node>] {
        self.NodeProps().children.as_ref()
    }

    fn as_layoutable(&self) -> &dyn Layoutable {
        self as &dyn Layoutable
    }

    fn as_renderable(&self) -> &dyn Renderable {
        self as &dyn Renderable
    }
}

impl<T: 'static + XcDataType> Layoutable for CoreNodeBase<T> {
    default fn layout(
        &self,
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        layout_children(
            &self.NodeProps().children,
            pango_context,
            style_computed,
            content_boundary,
        )
    }

    default fn display(&self) -> crate::style::Display {
        crate::style::Display::Block
    }
}

impl<T: 'static + XcDataType> Renderable for CoreNodeBase<T> {
    default fn paint(
        &self,
        renderer: &crate::rendering::cairo::CairoRenderer,
        style_computed: &Style,
    ) {
        paint_children(&self.NodeProps().children, renderer, style_computed)
    }
}

pub fn layout_children(
    children: &[Box<dyn Node>],
    pango_context: &pango::Context,
    style_computed: &Style,
    content_boundary: crate::common::Rectangle,
) -> Rectangle {
    let children: Vec<&dyn Layoutable> = children.iter().map(|c| c.as_layoutable()).collect();

    FlowLayout::layout(pango_context, style_computed, content_boundary, &children)
}

pub fn paint_children(
    children: &[Box<dyn Node>],
    renderer: &crate::rendering::cairo::CairoRenderer,
    style_computed: &Style,
) {
    children
        .iter()
        .map(|c| c.as_renderable())
        .for_each(|c| c.paint(renderer, style_computed))
}
