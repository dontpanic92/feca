use crosscom::ComRc;
use xcdt::XcDataType;

use super::{
    node::{CoreNode, CoreNodeBase},
    string::DomString,
};
use crate::comdef::{IDomString, IElementImpl};

xcdt::declare_xcdt!(CoreElement, ElementProps, CoreNode, CoreNodeBase);

pub struct ElementProps {
    id: ComRc<IDomString>,
    tag: ComRc<IDomString>,
}

impl ElementProps {
    pub fn new(id: ComRc<IDomString>, tag: &str) -> Self {
        Self {
            id,
            tag: DomString::new(tag.to_string()),
        }
    }
}

impl<T: 'static + XcDataType> IElementImpl for CoreElementBase<T> {
    fn id(&self) -> ComRc<IDomString> {
        self.ElementProps().id.clone()
    }

    fn tag(&self) -> crosscom::ComRc<IDomString> {
        self.ElementProps().tag.clone()
    }
}
