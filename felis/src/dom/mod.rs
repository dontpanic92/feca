use crosscom::ComRc;

use crate::defs::{INode, IDomString};

use self::html::html_element::Attributes;

pub mod core;
pub mod html;

pub struct NodeConstructor {
    name: String,
    ctor: fn(Vec<ComRc<INode>>, ComRc<IDomString>, Attributes) -> ComRc<INode>,
}
