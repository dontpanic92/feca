use crosscom::ComRc;

use crate::defs::IHtmlElement;

#[derive(Debug)]
pub struct IdSelector(pub String);

impl IdSelector {
    pub fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        element.id().str() == &self.0
    }
}

#[derive(Debug)]
pub struct TagSelector(pub String);

impl TagSelector {
    pub fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        element.tag().str() == &self.0
    }
}

#[derive(Debug)]
pub struct ClassSelector(pub String);

impl ClassSelector {
    pub fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        let class = element.get_attribute("class").flatten();
        if let Some(class) = class {
            for cls in class.split(' ') {
                if cls == &self.0 {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug)]
pub enum TrivalSelector {
    IdSelector(IdSelector),
    TagSelector(TagSelector),
    ClassSelector(ClassSelector),
}

impl TrivalSelector {
    pub fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        match self {
            TrivalSelector::IdSelector(s) => s.match_element(element),
            TrivalSelector::TagSelector(s) => s.match_element(element),
            TrivalSelector::ClassSelector(s) => s.match_element(element),
        }
    }
}
