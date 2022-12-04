use nom::{
    bytes::complete::{tag, take_until},
    combinator::{recognize, value},
    error::ParseError,
    sequence::tuple,
    IResult,
};

pub fn parse_c_style_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &str, E> {
    recognize(tuple((tag("/*"), take_until("*/"), tag("*/"))))(i)
}
