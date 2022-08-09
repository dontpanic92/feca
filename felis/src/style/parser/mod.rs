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

use super::{
    block::StyleBlock,
    selector::{ClassSelector, IdSelector, TagSelector, TrivalSelector},
    Style,
};

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

pub fn parse_inline(input: &str) -> IResult<&str, Style> {
    style_inline(input)
}

fn key(input: &str) -> IResult<&str, &str> {
    w(recognize(pair(
        alt((alpha1, tag("-"))),
        many0_count(alt((alphanumeric1, tag("-")))),
    )))(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(alpha1, many0_count(alt((alphanumeric1, tag("-"))))))(input)
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

fn style_inline(input: &str) -> IResult<&str, Style> {
    // let (input, _) = w(tag("{"))(input)?;
    // let (input, _) = multispace0(input)?;
    let (input, items) = many0(item)(input)?;
    // let (input, _) = w2(tag("}"))(input)?;

    println!("{:?}", &items);
    println!("{}", input);
    return Ok((input, Style::from_key_value_list(&items)));
}

fn class_selector(input: &str) -> IResult<&str, TrivalSelector> {
    let (input, _) = w(tag("."))(input)?;
    let (input, identifier) = identifier(input)?;

    Ok((
        input,
        TrivalSelector::ClassSelector(ClassSelector {
            0: identifier.to_string(),
        }),
    ))
}

fn tag_selector(input: &str) -> IResult<&str, TrivalSelector> {
    let (input, identifier) = identifier(input)?;

    Ok((
        input,
        TrivalSelector::TagSelector(TagSelector {
            0: identifier.to_string(),
        }),
    ))
}

fn id_selector(input: &str) -> IResult<&str, TrivalSelector> {
    let (input, _) = w(tag("#"))(input)?;
    let (input, identifier) = identifier(input)?;

    Ok((
        input,
        TrivalSelector::IdSelector(IdSelector {
            0: identifier.to_string(),
        }),
    ))
}

fn selector(input: &str) -> IResult<&str, TrivalSelector> {
    let (input, selector) = alt((class_selector, id_selector, tag_selector))(input)?;

    Ok((input, selector))
}

fn parse_block(input: &str) -> IResult<&str, StyleBlock> {
    let (input, selectors) = many1(w(selector))(input)?;
    let (input, _) = w2(tag("{"))(input)?;
    let (input, style) = parse_inline(input)?;
    let (input, _) = w2(tag("}"))(input)?;

    Ok((input, StyleBlock { selectors, style }))
}

pub fn parse_style(input: &str) -> IResult<&str, Vec<StyleBlock>> {
    many0(parse_block)(input)
}
