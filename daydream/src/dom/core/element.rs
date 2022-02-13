use xcdt::XcDataType;

use super::node::{CoreNode, CoreNodeBase};
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
impl<T: XcDataType> Element for CoreElementBase<T> {
    fn id(&self) -> Option<&str> {
        self.ext().ext().properties().id.as_deref()
    }
}
