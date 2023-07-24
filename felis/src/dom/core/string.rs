use crosscom::ComRc;

use crate::comdef::{IDomString, IDomStringImpl};

// TODO: Should be UTF-16
pub struct DomString {
    pub string: String,
}

crate::ComObject_DomString!(crate::dom::core::string::DomString);

impl DomString {
    pub fn new(string: String) -> ComRc<IDomString> {
        ComRc::<IDomString>::from_object(Self { string })
    }
}

impl IDomStringImpl for DomString {
    fn bytes(&self) -> *const std::os::raw::c_uchar {
        todo!()
    }

    fn str(&self) -> crosscom::StaticStr {
        unsafe { &*(&self.string as *const String) }
    }
}
