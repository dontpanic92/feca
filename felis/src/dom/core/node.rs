use std::cell::RefCell;

use crate::{
    comdef::{IDomString, IElement, INode, INodeImpl, IRenderable},
    common::Rectangle,
    dom::html::HtmlDom,
    layout::flow::FlowLayout,
    style::Style,
};
use crosscom::{ComRc, IUnknown, ObjectArray};
use xcdt::{Object, ObjectBase, XcDataType};

use super::string::DomString;

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
    children: RefCell<ObjectArray<INode>>,
}

impl NodeProps {
    pub fn new(node_type: NodeType, children: Vec<ComRc<INode>>) -> Self {
        let children = children
            .into_iter()
            .map(|c| c.query_interface::<IUnknown>().unwrap())
            .collect();
        Self {
            node_type,
            children: RefCell::new(ObjectArray::new(children)),
        }
    }

    pub fn get_children(&self) -> ObjectArray<INode> {
        self.children.borrow().clone()
    }

    pub fn set_children(&self, children: ObjectArray<INode>) {
        self.children.replace(children);
    }
}

impl<T: 'static + XcDataType> INodeImpl for CoreNodeBase<T> {
    fn children(&self) -> ObjectArray<INode> {
        self.NodeProps().get_children()
    }

    fn inner_html(&self) -> ComRc<IDomString> {
        let mut frag_list = vec![];
        for i in 0..self.children().len() {
            frag_list.push(self.children().get(i).outer_html().str());
        }

        DomString::new(frag_list.join("\n"))
    }

    default fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) {
        let html = "<html>".to_string() + html.str() + "</html>";
        let tl_dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();
        let dom = HtmlDom::from_tl_dom(&tl_dom);
        let root = dom.root().unwrap();
        self.NodeProps().children.replace(root.children());
    }

    default fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        DomString::new("<>".to_string() + self.inner_html().str() + "</>")
    }

    fn get_elements_by_tag_name(&self, tag: ComRc<IDomString>) -> ObjectArray<IElement> {
        let mut elements = vec![];

        for i in 0..self.children().len() {
            let node = self.children().get(i);
            let element = node.query_interface::<IElement>();
            if let Some(element) = element {
                if element.tag().str() == tag.str() {
                    elements.push(
                        self.children()
                            .get(i)
                            .query_interface::<IUnknown>()
                            .unwrap(),
                    );
                } else {
                    elements.extend_from_slice(node.get_elements_by_tag_name(tag.clone()).raw());
                }
            }
        }

        ObjectArray::new(elements)
    }

    fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<IElement>> {
        for i in 0..self.children().len() {
            let node = self.children().get(i);
            let element = node.query_interface::<IElement>();
            if let Some(element) = element {
                if element.id().str() == id.str() {
                    return Some(element);
                } else {
                    let ret = node.get_element_by_id(id.clone());
                    if ret.is_some() {
                        return ret;
                    }
                }
            }
        }

        None
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
