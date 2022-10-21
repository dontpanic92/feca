use nom::{
    bytes::complete::{tag, take_until},
    combinator::value,
    error::ParseError,
    sequence::tuple,
    IResult,
};

pub fn parse_c_style_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        tuple((tag("/*"), take_until("*/"), tag("*)"))),
    )(i)
}
