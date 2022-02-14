use crate::{
    dom::Node,
    layout::{flow::FlowLayout, Layoutable},
    rendering::Renderable,
};
use intertrait::castable_to;
use xcdt::{Object, ObjectBase};

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

pub(crate) trait NodeImpl: IsCoreNode {
    fn children(&self) -> &[Box<dyn Node>] {
        self.props().children.as_ref()
    }
}

impl<T: 'static + NodeImpl + Layoutable + Renderable> Node for T {
    fn children(&self) -> &[Box<dyn Node>] {
        NodeImpl::children(self)
    }

    fn as_layoutable(&self) -> &dyn Layoutable {
        self as &dyn Layoutable
    }

    fn as_renderable(&self) -> &dyn Renderable {
        self as &dyn Renderable
    }
}

pub(crate) trait LayoutImpl: IsCoreNode {
    fn layout(
        &self,
        pango_context: &pango::Context,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        let children: Vec<&dyn Layoutable> = self
            .props()
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

impl<T: 'static + LayoutImpl> Layoutable for T {
    fn layout(
        &self,
        pango_context: &pango::Context,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        LayoutImpl::layout(self, pango_context, content_boundary)
    }
}

pub(crate) trait RenderImpl: IsCoreNode {
    fn paint(&self, renderer: &crate::rendering::cairo::CairoRenderer) {
        <dyn RenderImpl>::paint_children(self.props().children(), renderer)
    }
}

impl dyn RenderImpl {
    fn paint_children(
        children: &[Box<dyn Node>],
        renderer: &crate::rendering::cairo::CairoRenderer,
    ) {
        children
            .iter()
            .map(|c| c.as_renderable())
            .for_each(|c| c.paint(renderer))
    }
}

impl<T: 'static + RenderImpl> Renderable for T {
    fn paint(&self, renderer: &crate::rendering::cairo::CairoRenderer) {
        RenderImpl::paint(self, renderer)
    }
}

impl NodeImpl for CoreNode {}
impl LayoutImpl for CoreNode {}
impl RenderImpl for CoreNode {}

castable_to!(CoreNode => Node, Layoutable);
