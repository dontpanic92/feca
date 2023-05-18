#![feature(min_specialization)]

pub use crate::rendering::cairo::CairoRenderer;
pub use dom::core::string::DomString;

mod common;
pub mod defs;
mod parser;
mod dom;
mod layout;
pub mod page;
mod rendering;
mod style;
