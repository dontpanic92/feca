mod comment;
mod string;

pub use comment::*;
pub use string::*;

use nom::{
    character::complete::multispace0,
    combinator::map,
    error::ParseError,
    sequence::{delimited, preceded},
    IResult,
};

pub fn w<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(multispace0, inner)
}

pub fn w2<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn boxed<'a, F: 'a, O, I, E: ParseError<I>>(inner: F) -> impl FnMut(I) -> IResult<I, Box<O>, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
{
    map(inner, |x| Box::new(x))
}

pub fn make_vec<'a, F: 'a, O, U, I, E: ParseError<I>>(
    inner: F,
) -> impl FnMut(I) -> IResult<I, Vec<U>, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
{
    map(inner, |_x| vec![])
}

pub fn to_string<'a, F: 'a, O, I, E: ParseError<I>>(
    inner: F,
) -> impl FnMut(I) -> IResult<I, String, E>
where
    O: Into<String>,
    F: FnMut(I) -> IResult<I, O, E>,
{
    map(inner, |x| x.into())
}
