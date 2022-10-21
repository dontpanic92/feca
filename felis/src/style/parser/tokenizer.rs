use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    multi::{many0, many0_count},
    sequence::pair,
    IResult,
};
use parser_utils::{parse_c_style_comment, w};

pub enum Token {
    Identifier(String),
    LeftBrace,
    RightBrace,
    Colon,
    Comment,
}

macro_rules! token {
    ($name: ident, $ret: expr, $parser: expr $(,)?) => {
        fn $name(input: &str) -> IResult<&str, Token> {
            // println!("entering {}", stringify!($name));
            let (input, _) = w($parser)(input)?;

            // println!("leaving {}: {}", stringify!($name), input);
            return Ok((input, $ret));
        }
    };
}

pub fn tokenize(input: &str) -> IResult<&str, Vec<Token>> {
    many0(alt((identifier, left_brace, right_brace, colon, comment)))(input)
}

fn identifier(input: &str) -> IResult<&str, Token> {
    let (input, ident) = w(recognize(pair(
        alt((alpha1, tag("_"), tag("_"), tag("@"))),
        many0_count(alt((alphanumeric1, tag("-")))),
    )))(input)?;

    Ok((input, Token::Identifier(ident.to_string())))
}

token!(left_brace, Token::LeftBrace, tag("{"));
token!(right_brace, Token::RightBrace, tag("}"));
token!(colon, Token::Colon, tag(":"));
token!(comment, Token::Comment, parse_c_style_comment);
