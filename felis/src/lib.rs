#![feature(generic_associated_types)]
#![feature(min_specialization)]

pub use crate::rendering::cairo::CairoRenderer;
pub use dom::core::string::DomString;
pub use page::FelisAction;
pub use page::Page;

mod common;
pub mod defs;
mod dom;
mod layout;
mod page;
mod rendering;
mod style;
