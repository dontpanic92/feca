use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::recognize,
    error::ParseError,
    multi::{many0, many0_count, many1, many1_count},
    sequence::{delimited, pair, preceded},
    IResult,
};

use super::Style;

fn w<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(multispace0, inner)
}

fn w2<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn parse(input: &str) -> IResult<&str, Style> {
    style(input)
}

fn key(input: &str) -> IResult<&str, &str> {
    w(recognize(pair(
        alt((alpha1, tag("-"))),
        many0_count(alt((alphanumeric1, tag("-")))),
    )))(input)
}

fn value(input: &str) -> IResult<&str, &str> {
    delimited(
        w2(tag(":")),
        recognize(many1_count(alt((alphanumeric1, tag("-"))))),
        w2(tag(";")),
    )(input)
}

fn item(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, key) = key(input)?;
    let (input, value) = value(input)?;

    Ok((input, (key, value)))
}

fn style(input: &str) -> IResult<&str, Style> {
    // let (input, _) = w(tag("{"))(input)?;
    // let (input, _) = multispace0(input)?;
    let (input, items) = many0(item)(input)?;
    // let (input, _) = w2(tag("}"))(input)?;

    println!("{:?}", &items);
    println!("{}", input);
    return Ok((input, Style::from_key_value_list(&items)));
}
