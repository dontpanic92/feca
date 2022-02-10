use crate::dom::Element;

use super::{node::CoreNodeBase, Nil, XcDataType};

crate::declare_xcdt!(CoreElement, ElementProps, CoreNodeBase);

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
        self.as_CoreElementBase().properties().id.clone()
    }
}

impl<T: XcDataType> XcCoreElement<T> {
    pub fn test(&self) {
        println!("{}", 2);
    }
}
