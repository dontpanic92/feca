mod tokenizer;

use std::cell::RefCell;

use nom::{
    branch::alt,
    bytes::complete::take,
    error::ParseError,
    multi::{many0, many1},
    sequence::delimited,
    IResult,
};

use self::tokenizer::{tokenize, Token, Tokens};

use super::{
    block::StyleBlock,
    selector::{ClassSelector, IdSelector, TagSelector, TrivalSelector},
    Style,
};

#[derive(Debug)]
pub enum ParsingError {
    LexerError(String),
    ParserError(Token),
    ParserError2,
}

impl ParsingError {
    pub fn short_print_err(&self) {
        match self {
            Self::LexerError(e) => println!("Lexer Error {}", e),
            Self::ParserError(e) => println!("Parser Error {:?}", e),
            Self::ParserError2 => println!("errr2"),
        }
    }
}

pub fn parse_style(input: &str) -> Result<Vec<StyleBlock>, ParsingError> {
    let (input, tokens) = tokenize(input).map_err(|err| {
        println!("after tokenize: {:?}", err);
        match err {
            nom::Err::Error(e) => ParsingError::LexerError(e.input.to_string()),
            nom::Err::Failure(e) => ParsingError::LexerError(e.input.to_string()),
            nom::Err::Incomplete(_) => ParsingError::LexerError("".to_string()),
        }
    })?;

    if input.trim().len() != 0 {
        return Err(ParsingError::LexerError(input.to_string()));
    }

    let (remaining, blocks) =
        parse_style_from_tokens(Tokens::new(&tokens)).map_err(|err| match err {
            nom::Err::Error(e) => ParsingError::ParserError(e.input.first().unwrap().clone()),
            nom::Err::Failure(e) => ParsingError::ParserError(e.input.first().unwrap().clone()),
            _ => ParsingError::ParserError2,
        })?;

    if remaining.count > 0 {
        return Err(ParsingError::ParserError(
            remaining.first().unwrap().clone(),
        ));
    }

    Ok(blocks)
}

pub fn parse_inline(input: &str) -> Result<Style, ParsingError> {
    let (input, tokens) = tokenize(input).map_err(|err| match err {
        nom::Err::Error(e) => ParsingError::LexerError(e.input.to_string()),
        nom::Err::Failure(e) => ParsingError::LexerError(e.input.to_string()),
        _ => ParsingError::LexerError("".to_string()),
    })?;

    if input.len() != 0 {
        return Err(ParsingError::LexerError(input.to_string()));
    }

    let (remaining, style) =
        parse_inline_from_tokens(Tokens::new(&tokens)).map_err(|err| match err {
            nom::Err::Error(e) => ParsingError::ParserError(e.input.first().unwrap().clone()),
            nom::Err::Failure(e) => ParsingError::ParserError(e.input.first().unwrap().clone()),
            _ => ParsingError::ParserError2,
        })?;

    if remaining.count > 0 {
        return Err(ParsingError::ParserError(
            remaining.first().unwrap().clone(),
        ));
    }

    Ok(style)
}

pub fn parse_inline_from_tokens(input: Tokens) -> IResult<Tokens, Style> {
    style_inline(input)
}

pub fn parse_style_from_tokens(input: Tokens) -> IResult<Tokens, Vec<StyleBlock>> {
    let parser = Parser::new();
    parser.style(input)
    /*ret.and_then(|ret| Ok((ret.0, ret.1)))
    .or_else(|err| Err(err.map(|e| nom::error::Error::new(e.input, e.code))))*/
}

macro_rules! make_tag {
    ($name: ident, $case: expr) => {
        fn $name(input: Tokens) -> IResult<Tokens, Token> {
            // recognize(pair(alpha1, many0_count(alt((alphanumeric1, tag("-"))))))(input)
            let (ident, input) = take(1usize)(input)?;
            if Some(&$case) == ident.first() {
                Ok((input, ident.first().unwrap().clone()))
            } else {
                Err(nom::Err::Error(nom::error::Error::from_error_kind(
                    input,
                    nom::error::ErrorKind::Tag,
                )))
            }
        }
    };
}

fn identifier(input: Tokens) -> IResult<Tokens, String> {
    let (input, ident) = take(1usize)(input)?;
    if let Some(Token::Identifier(ident)) = ident.first() {
        Ok((input, ident.clone()))
    } else {
        Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

fn tag_ident<'a>(tag: &str, input: Tokens<'a>) -> IResult<Tokens<'a>, String> {
    let (input, ident) = identifier(input)?;
    if ident.as_str() == tag {
        Ok((input, ident))
    } else {
        Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

fn string_literal(input: Tokens) -> IResult<Tokens, String> {
    let (input, token) = take(1usize)(input)?;
    if let Some(Token::StringLiteral(strlit)) = token.first() {
        Ok((input, strlit.clone()))
    } else {
        Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

make_tag!(tag_colon, Token::Colon);
make_tag!(tag_semicolon, Token::SemiColon);
make_tag!(tag_hash, Token::Hash);
make_tag!(tag_dot, Token::Dot);
make_tag!(tag_lbrace, Token::LeftBrace);
make_tag!(tag_rbrace, Token::RightBrace);

fn key(input: Tokens) -> IResult<Tokens, String> {
    identifier(input)
}

fn value(input: Tokens) -> IResult<Tokens, String> {
    delimited(tag_colon, identifier, tag_semicolon)(input)
}

fn item(input: Tokens) -> IResult<Tokens, (String, String)> {
    let (input, key) = key(input)?;
    let (input, value) = value(input)?;

    Ok((input, (key, value)))
}

fn style_inline(input: Tokens) -> IResult<Tokens, Style> {
    // let (input, _) = w(tag("{"))(input)?;
    // let (input, _) = multispace0(input)?;
    let (input, items) = many0(item)(input)?;
    // let (input, _) = w2(tag("}"))(input)?;

    // println!("{:?}", &items);
    // println!("{}", input);
    return Ok((input, Style::from_key_value_list(&items)));
}

fn class_selector(input: Tokens) -> IResult<Tokens, TrivalSelector> {
    let (input, _) = tag_dot(input)?;
    let (input, identifier) = identifier(input)?;

    Ok((
        input,
        TrivalSelector::ClassSelector(ClassSelector { 0: identifier }),
    ))
}

fn tag_selector(input: Tokens) -> IResult<Tokens, TrivalSelector> {
    let (input, identifier) = identifier(input)?;

    Ok((
        input,
        TrivalSelector::TagSelector(TagSelector { 0: identifier }),
    ))
}

fn id_selector(input: Tokens) -> IResult<Tokens, TrivalSelector> {
    let (input, _) = tag_hash(input)?;
    let (input, identifier) = identifier(input)?;

    Ok((
        input,
        TrivalSelector::IdSelector(IdSelector {
            0: identifier.to_string(),
        }),
    ))
}

fn selector(input: Tokens) -> IResult<Tokens, TrivalSelector> {
    let (input, selector) = alt((class_selector, id_selector, tag_selector))(input)?;

    Ok((input, selector))
}

struct ParserContext {
    charset: String,
}

impl ParserContext {
    pub fn new() -> Self {
        Self {
            charset: "utf-8".to_string(),
        }
    }
}

struct Parser {
    context: RefCell<ParserContext>,
}

impl Parser {
    fn new() -> Self {
        Self {
            context: RefCell::new(ParserContext::new()),
        }
    }

    fn regulat_at_rule_charset<'a>(input: Tokens<'a>) -> IResult<Tokens<'a>, ()> {
        let (input, _) = tag_ident("@charset", input)?;
        let (input, _) = string_literal(input)?;
        let (input, _) = tag_semicolon(input)?;

        Ok((input, ()))
    }

    fn regular_at_rule<'a>(&self, input: Tokens<'a>) -> IResult<Tokens<'a>, Option<StyleBlock>> {
        let (input, _) = Self::regulat_at_rule_charset(input)?;
        Ok((input, None))
    }

    fn parse_block<'a>(&self, input: Tokens<'a>) -> IResult<Tokens<'a>, Option<StyleBlock>> {
        let (input, selectors) = many1(selector)(input)?;
        let (input, _) = tag_lbrace(input)?;
        let (input, style) = parse_inline_from_tokens(input)?;
        let (input, _) = tag_rbrace(input)?;

        Ok((input, Some(StyleBlock { selectors, style })))
    }

    fn style<'a>(&self, tokens: Tokens<'a>) -> IResult<Tokens<'a>, Vec<StyleBlock>> {
        let (input, ret) = many0(alt((
            self.bind(Self::regular_at_rule),
            self.bind(Self::parse_block),
        )))(tokens)?;

        Ok((input, ret.into_iter().flatten().collect()))
    }

    fn bind<'s, 'a, R, F: 's + Fn(&Self, Tokens<'a>) -> IResult<Tokens<'a>, R>>(
        &'s self,
        func: F,
    ) -> impl 's + Fn(Tokens<'a>) -> IResult<Tokens<'a>, R> {
        move |s: Tokens| func(self, s)
    }
}

/*
fn bind<'a, 'b, R, F: 'b + Fn(&'a str, &'b ParseContext) -> IResult<&'a str, R>>(func: F, context: &'b ParseContext) -> impl 'b + Fn(&'a str)  -> IResult<&'a str, R> {
    move |s: &'a str| { func(s, context) }
}
*/
