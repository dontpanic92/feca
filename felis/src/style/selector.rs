
pub struct IdSelector(pub String);
pub struct TagSelector(pub String);
pub struct ClassSelector(pub String);

pub enum TrivalSelector {
    IdSelector(IdSelector),
    TagSelector(TagSelector),
    ClassSelector(ClassSelector),
}
