use super::node::{CoreNode, CoreNodeBase};
use crate::dom::Element;
use xcdt::XcDataType;

xcdt::declare_xcdt!(CoreElement, ElementProps, CoreNode, CoreNodeBase);

pub struct ElementProps {
    id: String,
}

impl ElementProps {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
impl<T: XcDataType> Element for CoreElementBase<T> {
    fn id(&self) -> String {
        self.ext().ext().properties().id.clone()
    }
}
