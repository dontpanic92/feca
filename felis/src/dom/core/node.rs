use crate::{
    common::Rectangle,
    defs::{IDomString, INode, INodeImpl, IRenderable},
    layout::flow::FlowLayout,
    style::Style,
};
use crosscom::{ComRc, IUnknown, ObjectArray};
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

pub struct Node(pub CoreNode);

pub struct NodeProps {
    node_type: NodeType,
    children: ObjectArray<INode>,
}

impl NodeProps {
    pub fn new(node_type: NodeType, children: Vec<ComRc<INode>>) -> Self {
        let children = children
            .into_iter()
            .map(|c| c.query_interface::<IUnknown>().unwrap())
            .collect();
        Self {
            node_type,
            children: ObjectArray::new(children),
        }
    }

    pub fn children(&self) -> ObjectArray<INode> {
        self.children.clone()
    }
}

impl<T: 'static + XcDataType> INodeImpl for CoreNodeBase<T> {
    fn children(&self) -> ObjectArray<INode> {
        self.NodeProps().children.clone()
    }

    fn inner_html(&self) -> ComRc<IDomString> {
        // let mut frag_list = vec![];
        for i in 0..self.NodeProps().children.len() {
            self.NodeProps().children.get(i).children();
        }

        // frag_list.join("\n")
        todo!();
    }
}

pub fn layout_children(
    children: ObjectArray<INode>,
    pango_context: &pango::Context,
    style_computed: &Style,
    content_boundary: crate::common::Rectangle,
) -> Rectangle {
    let children: Vec<ComRc<IRenderable>> = children
        .raw()
        .iter()
        .flat_map(|c| c.query_interface::<IRenderable>())
        .collect();

    FlowLayout::layout(pango_context, style_computed, content_boundary, &children)
}

pub fn paint_children(
    children: ObjectArray<INode>,
    renderer: &crate::rendering::cairo::CairoRenderer,
    style_computed: &Style,
) {
    children
        .raw()
        .iter()
        .flat_map(|c| c.query_interface::<IRenderable>())
        .for_each(|c| c.paint(renderer, style_computed))
}
