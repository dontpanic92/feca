use crosscom::ComRc;

use crate::defs::IHtmlElement;

pub trait IBasicSelector {
    fn match_element(&self, element: ComRc<IHtmlElement>) -> bool;
}

#[derive(Debug)]
pub struct IdSelector(pub String);

impl IBasicSelector for IdSelector {
    fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        element.id().str() == &self.0
    }
}

#[derive(Debug)]
pub struct TypeSelector(pub String);

impl IBasicSelector for TypeSelector {
    fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        element.tag().str() == &self.0
    }
}

#[derive(Debug)]
pub struct ClassSelector(pub String);

impl IBasicSelector for ClassSelector {
    fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
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
pub struct AttributeSelector(pub String);

impl IBasicSelector for AttributeSelector {
    fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct PseudoClassSelector(pub String);

impl IBasicSelector for PseudoClassSelector {
    fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct PseudoElementSelector(pub String);

impl IBasicSelector for PseudoElementSelector {
    fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        false
    }
}

/*
#[derive(Debug)]
pub struct WithPseudo<T: IBasicSelector> {
    selector: T,
    pseudo_classes: Vec<String>,
    pseudo_element: Option<String>,
}

impl<T: IBasicSelector> WithPseudo<T> {
    pub fn new(selector: T, pseudo_classes: Vec<String>, pseudo_element: Option<String>) -> Self {
        Self {
            selector,
            pseudo_classes,
            pseudo_element,
        }
    }
}

impl<T: IBasicSelector> WithPseudo<T> {
    fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        self.selector.match_element(element)
    }
}*/

#[derive(Debug)]
pub enum BasicSelector {
    Universal(IdSelector),
    Id(IdSelector),
    Type(TypeSelector),
    Class(ClassSelector),
    Attribute(AttributeSelector),
    PseudoClass(PseudoClassSelector),
    PseudoElement(PseudoElementSelector),
}

impl BasicSelector {
    pub fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        match self {
            BasicSelector::Universal(_) => true,
            BasicSelector::Id(s) => s.match_element(element),
            BasicSelector::Type(s) => s.match_element(element),
            BasicSelector::Class(s) => s.match_element(element),
            BasicSelector::Attribute(s) => s.match_element(element),
            BasicSelector::PseudoClass(s) => s.match_element(element),
            BasicSelector::PseudoElement(s) => s.match_element(element),
        }
    }
}

#[derive(Debug)]
pub enum SelectorCombinator {
    Basic(BasicSelector),
    Descendant(BasicSelector, Box<SelectorCombinator>),
    Child(BasicSelector, Box<SelectorCombinator>),
    GeneralSibling(BasicSelector, Box<SelectorCombinator>),
    AdjacentSibling(BasicSelector, Box<SelectorCombinator>),
}

impl SelectorCombinator {
    pub fn match_element(&self, element: ComRc<IHtmlElement>) -> bool {
        match self {
            Self::Basic(s) => s.match_element(element),
            Self::Descendant(s, _) => s.match_element(element),
            Self::Child(s, _) => s.match_element(element),
            Self::GeneralSibling(s, _) => s.match_element(element),
            Self::AdjacentSibling(s, _) => s.match_element(element),
        }
    }
}
