#![feature(min_specialization)]

pub use crate::rendering::cairo::CairoRenderer;
pub use dom::core::string::DomString;

pub mod comdef;
pub mod common;
pub mod dom;
mod layout;
pub mod page;
mod parser;
pub mod rendering;
pub mod style;
