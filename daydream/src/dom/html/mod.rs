use super::{core::text, Element, Node};

mod body;
mod html_element;
mod paragraph;

pub(crate) trait HtmlElement: Element {
    fn title(&self) -> Option<&str>;
}

pub(crate) trait Paragraph: HtmlElement {}

#[derive(Debug)]
pub(crate) struct HtmlDom {
    root: Option<Box<dyn Node>>,
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

    pub fn process_tl_node(tl_node: &tl::Node, tl_parser: &tl::Parser) -> Option<Box<dyn Node>> {
        match tl_node {
            tl::Node::Tag(t) => {
                println!("tag: {}", t.name().as_utf8_str());
                let children: Vec<Box<dyn Node>> = t
                    .children()
                    .top()
                    .iter()
                    .flat_map(|c| Self::process_tl_node(c.get(tl_parser)?, tl_parser))
                    .collect();

                match t.name().as_utf8_str().to_lowercase().as_str() {
                    "body" => Some(body::new_core_body(children)),
                    "p" => Some(paragraph::new_core_paragraph(children)),
                    _ => None,
                }
            }
            tl::Node::Raw(b) => Some(text::new_core_text(b.as_utf8_str().to_string())),
            tl::Node::Comment(b) => None?,
        }
    }
}
