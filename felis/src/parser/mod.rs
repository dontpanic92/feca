use crosscom::ComRc;
use nom::IResult;

use crate::{dom::html::HtmlDom, defs::INode};


pub(crate) fn parse(input: &str) -> IResult<&str, HtmlDom> {
    let (input, r) = root(input)?;
    Ok((input, HtmlDom::from_root(Some(r))))
}

fn root(input: &str) -> IResult<&str, ComRc<INode>> {
    unimplemented!()
}
