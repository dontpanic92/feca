use super::node::{CoreNode, CoreNodeBase, IsCoreNode, LayoutImpl, NodeImpl, RenderImpl};
use crate::dom::Element;

xcdt::declare_xcdt!(CoreElement, ElementProps, CoreNode, CoreNodeBase);

pub struct ElementProps {
    id: Option<String>,
}

impl ElementProps {
    pub fn new(id: Option<String>) -> Self {
        Self { id }
    }
}

pub(crate) trait ElementImpl: IsCoreNode + IsCoreElement {
    fn id(&self) -> Option<&str> {
        IsCoreElement::props(self).id.as_deref()
    }
}

impl NodeImpl for CoreElement {}
impl LayoutImpl for CoreElement {}
impl RenderImpl for CoreElement {}
impl ElementImpl for CoreElement {}

impl<T: ElementImpl> Element for T {
    fn id(&self) -> Option<&str> {
        ElementImpl::id(self)
    }
}
