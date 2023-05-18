use std::collections::HashMap;

use crosscom::ComRc;
use dashmap::DashMap;

use crate::{
    defs::{IDomString, INode},
    dom::html::html_element::Attributes,
};

use super::core::{string::DomString, text};

pub mod body;
pub mod div;
pub mod head;
pub mod html;
pub mod html_element;
pub mod img;
pub mod link;
pub mod paragraph;
pub mod script;
pub mod style;

lazy_static::lazy_static! {
    static ref TAG_CTOR_MAP: DashMap<String, fn(
        Vec<ComRc<INode>>,
        ComRc<IDomString>,
        Attributes,
    ) -> ComRc<INode>> = {
        let map = DashMap::new();
        map.insert("html".to_string(), html::new_core_html as fn(
            Vec<ComRc<INode>>,
            ComRc<IDomString>,
            Attributes,
        ) -> ComRc<INode>);

        macro_rules! tag {
            ($name: ident) => {
                tag!($name, $name);
            };

            ($name: ident, $module: ident) => {
                paste::paste! {
                    map.insert(stringify!($name).to_string(), $module::[<new_ $name >]);
                }
            };
        }
        tag!(head);
        tag!(body);
        tag!(p, paragraph);
        tag!(i, html_element);
        tag!(a, html_element);
        tag!(b, html_element);
        tag!(h1, html_element);
        tag!(script);
        tag!(style);
        tag!(div);
        tag!(img);
        tag!(link);
        map
    };
}

pub(crate) struct HtmlDom {
    root: Option<ComRc<INode>>,
}

impl HtmlDom {
    pub fn from_tl_dom(tl_dom: &tl::VDom) -> Self {
        let root = {
            let parser = tl_dom.parser();
            let root = tl_dom
                .children()
                .iter()
                .find(|n| Self::is_html_node(n.get(parser).unwrap()))
                .unwrap()
                .get(parser);
            root.and_then(|r| Self::process_tl_node(r, parser))
        };

        Self { root }
    }

    pub fn from_root(root: Option<ComRc<INode>>) -> Self {
        Self { root }
    }

    pub fn root(&self) -> Option<ComRc<INode>> {
        self.root.clone()
    }

    fn is_html_node(tl_node: &tl::Node) -> bool {
        match tl_node {
            tl::Node::Tag(t) => t.name().as_utf8_str().to_lowercase() == "html",
            _ => false,
        }
    }

    fn process_tl_node(tl_node: &tl::Node, tl_parser: &tl::Parser) -> Option<ComRc<INode>> {
        match tl_node {
            tl::Node::Tag(t) => {
                let children: Vec<ComRc<INode>> = t
                    .children()
                    .top()
                    .iter()
                    .flat_map(|c| Self::process_tl_node(c.get(tl_parser)?, tl_parser))
                    .collect();
                let id = DomString::new(
                    t.attributes()
                        .id()
                        .map(|id| std::str::from_utf8(id.as_bytes()).unwrap_or("").to_string())
                        .unwrap_or("".to_string()),
                );

                let attributes = t
                    .attributes()
                    .iter()
                    .map(|attr| (attr.0.into_owned(), attr.1.map(|str| str.into_owned())))
                    .collect::<HashMap<String, Option<String>>>();

                let tag_name = t.name().as_utf8_str().to_lowercase();
                println!("process tag node: {:?}", tag_name);
                if let Some(ctor) = TAG_CTOR_MAP.get(tag_name.as_str()) {
                    Some(ctor(children, id, attributes))
                } else {
                    // None
                    Some(TAG_CTOR_MAP.get("div").unwrap()(children, id, attributes))
                }
            }
            tl::Node::Raw(b) => {
                let s = b.as_utf8_str();
                if s.trim() != "" {
                    Some(text::new_core_text(s.to_string()))
                } else {
                    None
                }
            }
            tl::Node::Comment(_) => None,
        }
    }
}
