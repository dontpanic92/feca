use crosscom::ComRc;

use crate::defs::INode;

use super::core::{string::DomString, text};

pub mod body;
pub mod head;
pub mod html;
pub mod html_element;
pub mod paragraph;
pub mod script;

pub(crate) struct HtmlDom {
    root: Option<ComRc<INode>>,
}

impl HtmlDom {
    pub fn from_tl_dom(tl_dom: &tl::VDom) -> Self {
        let root = {
            let parser = tl_dom.parser();
            let root = tl_dom.children()[0].get(parser);
            root.and_then(|r| Self::process_tl_node(r, parser))
        };

        Self { root }
    }

    pub fn root(&self) -> Option<ComRc<INode>> {
        self.root.clone()
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

                match t.name().as_utf8_str().to_lowercase().as_str() {
                    "html" => Some(html::new_core_html(children, id)),
                    "head" => Some(head::new_core_head(children, id)),
                    "body" => Some(body::new_core_body(children, id)),
                    "p" => Some(paragraph::new_core_paragraph(children, id)),
                    "i" => Some(html_element::new_i_element(children, id)),
                    "a" => Some(html_element::new_a_element(children, id)),
                    "b" => Some(html_element::new_b_element(children, id)),
                    "h1" => Some(html_element::new_h1_element(children, id)),
                    "script" => Some(script::new_core_script(children, id)),
                    _ => None,
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
