use crosscom::ComRc;
use nom::IResult;

use crate::{comdef::INode, dom::html::HtmlDom};

pub(crate) fn parse(input: &str) -> IResult<&str, HtmlDom> {
    let (input, r) = root(input)?;
    Ok((input, HtmlDom::from_root(Some(r))))
}

fn root(_input: &str) -> IResult<&str, ComRc<INode>> {
    unimplemented!()
}
