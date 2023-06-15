#![feature(min_specialization)]

pub use crate::rendering::cairo::CairoRenderer;
pub use dom::core::string::DomString;

mod common;
pub mod defs;
mod dom;
mod layout;
pub mod page;
mod parser;
mod rendering;
mod style;
