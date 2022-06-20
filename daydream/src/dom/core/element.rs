use super::node::{CoreNode, CoreNodeBase, IsCoreNode, NodeImpl};
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

pub(crate) trait ElementImpl: IsCoreElement {}

impl<T: ElementImpl> Element for T {
    fn id(&self) -> Option<&str> {
        self.ElementProps().id.as_deref()
    }
}

impl NodeImpl for CoreElement {}
impl ElementImpl for CoreElement {}
