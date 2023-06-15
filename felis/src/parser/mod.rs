use crosscom::ComRc;
use nom::IResult;

use crate::{defs::INode, dom::html::HtmlDom};

pub(crate) fn parse(input: &str) -> IResult<&str, HtmlDom> {
    let (input, r) = root(input)?;
    Ok((input, HtmlDom::from_root(Some(r))))
}

fn root(input: &str) -> IResult<&str, ComRc<INode>> {
    unimplemented!()
}
