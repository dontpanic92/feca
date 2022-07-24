use xcdt::XcDataType;

use super::node::{CoreNode, CoreNodeBase};
use crate::dom::defs::{ComObject_Element, IElementImpl};

xcdt::declare_xcdt!(CoreElement, ElementProps, CoreNode, CoreNodeBase);

pub struct Element(pub CoreElement);
ComObject_Element!(crate::dom::core::element::Element);

pub struct ElementProps {
    id: Option<String>,
}

impl ElementProps {
    pub fn new(id: Option<String>) -> Self {
        Self { id }
    }
}

impl<T: 'static + XcDataType> IElementImpl for CoreElementBase<T> {
    fn id(&self) -> () {
        self.ElementProps().id.as_deref()
    }
}
