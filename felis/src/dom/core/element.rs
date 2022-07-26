use crosscom::ComRc;
use xcdt::XcDataType;

use super::node::{CoreNode, CoreNodeBase};
use crate::defs::{IDomString, IElementImpl};

xcdt::declare_xcdt!(CoreElement, ElementProps, CoreNode, CoreNodeBase);

pub struct ElementProps {
    id: ComRc<IDomString>,
}

impl ElementProps {
    pub fn new(id: ComRc<IDomString>) -> Self {
        Self { id }
    }
}

impl<T: 'static + XcDataType> IElementImpl for CoreElementBase<T> {
    fn id(&self) -> ComRc<IDomString> {
        self.ElementProps().id.clone()
    }
}
