use crosscom::ComRc;

use crate::comdef::{IDomString, INode};

use self::html::html_element::Attributes;

pub mod core;
pub mod html;

pub struct NodeConstructor {
    name: String,
    ctor: fn(Vec<ComRc<INode>>, ComRc<IDomString>, Attributes) -> ComRc<INode>,
}
