use crate::{dom::html::paragraph::{ImplTestViaCoreParagraph, new_core_paragraph}, layout::Test};

pub fn test() {
    let p = new_core_paragraph(vec![]);
    let q = p as Box<dyn ImplTestViaCoreParagraph>;
    q.test();
}
