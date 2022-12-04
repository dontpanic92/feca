use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    multi::{many0, many0_count},
    sequence::pair,
    IResult, InputIter, InputLength, InputTake,
};
use parser_utils::{parse_c_style_comment, parse_string, w};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tokens<'a> {
    pub tokens: &'a [Token],
    pub start: usize,
    pub count: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            start: 0,
            count: tokens.len(),
        }
    }

    pub fn first(&self) -> Option<&Token> {
        if self.count > 0 {
            Some(&self.tokens[self.start])
        } else {
            None
        }
    }
}

impl<'a> InputLength for Tokens<'a> {
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'a> InputTake for Tokens<'a> {
    fn take(&self, count: usize) -> Self {
        Self {
            tokens: &self.tokens[0..count],
            start: 0,
            count,
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (first, second) = self.tokens.split_at(count);

        (
            Self {
                tokens: second,
                start: 0,
                count: self.count - count,
            },
            Self {
                tokens: first,
                start: 0,
                count,
            },
        )
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = &'a Token;
    type Iter = std::iter::Enumerate<std::slice::Iter<'a, Token>>;
    type IterElem = std::slice::Iter<'a, Token>;

    fn iter_indices(&self) -> std::iter::Enumerate<::std::slice::Iter<'a, Token>> {
        self.tokens.iter().enumerate()
    }
    fn iter_elements(&self) -> ::std::slice::Iter<'a, Token> {
        self.tokens.iter()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.tokens.iter().position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
            Err(nom::Needed::Unknown)
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Identifier(String),
    StringLiteral(String),
    LeftBrace,
    RightBrace,
    Colon,
    SemiColon,
    Comment,
    Dot,
    Hash,
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
    many0(alt((
        identifier,
        left_brace,
        right_brace,
        colon,
        comment,
        semicolon,
        dot,
        hash,
        string_literal,
    )))(input)
}

fn identifier(input: &str) -> IResult<&str, Token> {
    let (input, ident) = w(recognize(pair(
        alt((alpha1, tag("_"), tag("-"), tag("@"))),
        many0_count(alt((alphanumeric1, tag("-")))),
    )))(input)?;

    Ok((input, Token::Identifier(ident.to_string())))
}

fn string_literal(input: &str) -> IResult<&str, Token> {
    let (input, s) = parse_string(input)?;
    Ok((input, Token::StringLiteral(s)))
}

token!(left_brace, Token::LeftBrace, tag("{"));
token!(right_brace, Token::RightBrace, tag("}"));
token!(colon, Token::Colon, tag(":"));
token!(dot, Token::Dot, tag("."));
token!(semicolon, Token::SemiColon, tag(";"));
token!(hash, Token::Hash, tag("#"));
token!(comment, Token::Comment, parse_c_style_comment);
