use crate::{
    dom::Node,
    layout::{flow::FlowLayout, Layoutable},
    rendering::Renderable,
};
use intertrait::castable_to;
use xcdt::{Object, ObjectBase, XcDataType};

xcdt::declare_xcdt!(CoreNode, NodeProps, Object, ObjectBase);

pub struct NodeProps {
    node_type: i32,
    children: Vec<Box<dyn Node>>,
}

impl NodeProps {
    pub fn new(node_type: i32, children: Vec<Box<dyn Node>>) -> Self {
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
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        let children: Vec<&dyn Layoutable> = self
            .NodeProps()
            .children()
            .iter()
            //.map(|c| CastRef::cast::<dyn Layoutable>(c))
            .map(|c| c.as_layoutable())
            //.filter(|c| c.is_some())
            //.map(|c| c.unwrap())
            .collect();

        FlowLayout::layout(pango_context, content_boundary, &children)
    }
}

impl<T: 'static + XcDataType> Renderable for CoreNodeBase<T> {
    default fn paint(&self, renderer: &crate::rendering::cairo::CairoRenderer) {
        paint_children(self.NodeProps().children(), renderer)
    }
}

fn paint_children(children: &[Box<dyn Node>], renderer: &crate::rendering::cairo::CairoRenderer) {
    children
        .iter()
        .map(|c| c.as_renderable())
        .for_each(|c| c.paint(renderer))
}

castable_to!(CoreNode => Node, Layoutable);
