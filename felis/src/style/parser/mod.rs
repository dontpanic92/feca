mod tokenizer;

use std::cell::RefCell;

use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::{map, opt},
    error::ParseError,
    multi::{many0, many1, separated_list0, separated_list1},
    IResult,
};

use crate::style::selector::TypeSelector;

use self::tokenizer::{tokenize, InputDebug, Token, Tokens};

use super::{
    block::StyleBlock,
    selector::{
        AttributeSelector, BasicSelector, ClassSelector, IdSelector, PseudoClassSelector,
        PseudoElementSelector, SelectorCombinator,
    },
    Style,
};

#[derive(Debug)]
pub enum ParsingError {
    LexerError(String),
    ParserError(Vec<Token>),
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
    let (input, tokens) = tokenize(input).map_err(|err| match err {
        nom::Err::Error(e) => ParsingError::LexerError(e.input.to_string()),
        nom::Err::Failure(e) => ParsingError::LexerError(e.input.to_string()),
        nom::Err::Incomplete(_) => ParsingError::LexerError("".to_string()),
    })?;

    if input.trim().len() != 0 {
        return Err(ParsingError::LexerError(input.to_string()));
    }

    let (remaining, blocks) =
        parse_style_from_tokens(Tokens::new(&tokens)).map_err(|err| match err {
            nom::Err::Error(e) => ParsingError::ParserError(e.input.take_vec(5)),
            nom::Err::Failure(e) => ParsingError::ParserError(e.input.take_vec(5)),
            _ => ParsingError::ParserError2,
        })?;

    if remaining.count > 0 {
        return Err(ParsingError::ParserError(remaining.take_vec(5)));
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

    let (remaining, style) = style_inline(Tokens::new(&tokens)).map_err(|err| match err {
        nom::Err::Error(e) => ParsingError::ParserError(e.input.take_vec(5)),
        nom::Err::Failure(e) => ParsingError::ParserError(e.input.take_vec(5)),
        _ => ParsingError::ParserError2,
    })?;

    if remaining.count > 0 {
        return Err(ParsingError::ParserError(remaining.take_vec(5)));
    }

    Ok(style)
}

pub fn parse_style_from_tokens(input: Tokens) -> IResult<Tokens, Vec<StyleBlock>> {
    let parser = Parser::new();
    parser.style(input)
}

macro_rules! make_tag {
    ($name: ident, $case: expr) => {
        fn $name(input: Tokens) -> IResult<Tokens, Token> {
            // recognize(pair(alpha1, many0_count(alt((alphanumeric1, tag("-"))))))(input)
            let (input, ident) = take(1usize)(input)?;
            // println!("tag {:?} {:?}", &$case, ident.first());
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
    let _a = InputDebug::print(&input, "identifier");
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
make_tag!(tag_double_colon, Token::DoubleColon);
make_tag!(tag_semicolon, Token::SemiColon);
make_tag!(tag_hash, Token::Hash);
make_tag!(tag_dot, Token::Dot);
make_tag!(tag_plus, Token::Plus);
make_tag!(tag_minus, Token::Minus);
make_tag!(tag_comma, Token::Comma);
make_tag!(tag_star, Token::Star);
make_tag!(tag_tilde, Token::Tilde);
make_tag!(tag_lt, Token::Lt);
make_tag!(tag_gt, Token::Gt);
make_tag!(tag_lbrace, Token::LeftBrace);
make_tag!(tag_rbrace, Token::RightBrace);
make_tag!(tag_lbracket, Token::LeftBracket);
make_tag!(tag_rbracket, Token::RightBracket);
make_tag!(tag_lparen, Token::LeftParen);
make_tag!(tag_rparen, Token::RightParen);

fn _take_until<F: Fn(Option<&Token>) -> bool>(
    mut input: Tokens,
    predicate: F,
) -> IResult<Tokens, ()> {
    loop {
        let token = input.first();
        if predicate(token) {
            break;
        }

        let ret = take(1usize)(input)?;
        input = ret.0;
    }

    Ok((input, ()))
}

fn key(input: Tokens) -> IResult<Tokens, String> {
    let _a = InputDebug::print(&input, "key");
    identifier(input)
}

fn mul_expression(mut input: Tokens) -> IResult<Tokens, ()> {
    (input, _) = primary_expression(input)?;

    while input.first() == Some(&Token::Star) || input.first() == Some(&Token::Slash) {
        (input, _) = take(1usize)(input)?;
        (input, _) = priority_expression(input)?;
    }

    Ok((input, ()))
}

fn add_expression(mut input: Tokens) -> IResult<Tokens, ()> {
    let _a = InputDebug::print(&input, "add_expression");
    (input, _) = mul_expression(input)?;
    while input.first() == Some(&Token::Plus) || input.first() == Some(&Token::Minus) {
        (input, _) = take(1usize)(input)?;
        (input, _) = mul_expression(input)?;
    }

    Ok((input, ()))
}

fn para_expression(input: Tokens) -> IResult<Tokens, ()> {
    let (input, _) = tag_lparen(input)?;
    let (input, _) = add_expression(input)?;
    let (input, _) = tag_rparen(input)?;

    Ok((input, ()))
}

fn priority_expression(input: Tokens) -> IResult<Tokens, ()> {
    alt((para_expression, add_expression))(input)
}

fn assign_expression(mut input: Tokens) -> IResult<Tokens, ()> {
    (input, _) = identifier(input)?;
    let peek = input.first();
    if peek == Some(&Token::Assign) || peek == Some(&Token::CaretEqual) {
        (input, _) = take(1usize)(input)?;
        (input, _) = priority_expression(input)?;
    }

    Ok((input, ()))
}

fn function_argument(input: Tokens) -> IResult<Tokens, ()> {
    let _a = InputDebug::print(&input, "function_argument");
    let (input, _) = many1(priority_expression)(input)?;
    Ok((input, ()))
}

fn function_call(input: Tokens) -> IResult<Tokens, ()> {
    let _a = InputDebug::print(&input, "functioncall");
    let (input, _) = identifier(input)?;
    let (input, _) = tag_lparen(input)?;
    let (input, _) = separated_list0(tag_comma, function_argument)(input)?;
    let (input, _) = tag_rparen(input)?;

    Ok((input, ()))
}

fn color(input: Tokens) -> IResult<Tokens, String> {
    let (input, _) = tag_hash(input)?;
    let (input, identifier) = identifier(input)?;

    Ok((input, identifier))
}

fn primary_expression(input: Tokens) -> IResult<Tokens, PropertyValue> {
    let _a = InputDebug::print(&input, "expression");
    alt((
        map(function_call, |_value| PropertyValue::FunctionCall),
        map(identifier, |value| PropertyValue::String(value)),
        map(color, |value| PropertyValue::Color(value)),
        map(string_literal, |value| PropertyValue::String(value)),
    ))(input)
}

pub enum PropertyValue {
    String(String),
    Color(String),
    FunctionCall,
}

fn property_value_compat(input: Tokens) -> IResult<Tokens, Vec<PropertyValue>> {
    let _a = InputDebug::print(&input, "property_value_compat");
    many1(primary_expression)(input)
}

fn property_value_list(input: Tokens) -> IResult<Tokens, Vec<Vec<PropertyValue>>> {
    let _a = InputDebug::print(&input, "property_value_list");
    separated_list1(tag_comma, property_value_compat)(input)
}

pub struct Property {
    pub key: String,
    pub value: Vec<Vec<PropertyValue>>,
}

fn property(input: Tokens) -> IResult<Tokens, Property> {
    let _a = InputDebug::print(&input, "property");
    let (input, key) = key(input)?;
    let (input, _) = tag_colon(input)?;
    let (input, value) = opt(property_value_list)(input)?;

    let peek = input.first();
    let input = match peek {
        Some(&Token::SemiColon) => {
            let (input, _) = take(1usize)(input)?;
            input
        }
        Some(&Token::RightBrace) => input,
        _ => {
            return Err(nom::Err::Error(nom::error::Error::from_error_kind(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    let value = value.unwrap_or(vec![]);

    Ok((input, Property { key, value }))
}

fn style_inline(input: Tokens) -> IResult<Tokens, Style> {
    let (input, items) = many0(property)(input)?;
    let ret = Ok((input, Style::from_property_list(&items)));
    ret
}

fn pseudo_class_single(input: Tokens) -> IResult<Tokens, String> {
    let _a = InputDebug::print(&input, "pseudo_class_single");
    let (input, _) = tag_colon(input)?;
    let (input, ident) = identifier(input)?;
    if input.first() == Some(&Token::LeftParen) {
        let (input, _) = tag_lparen(input)?;
        let (input, _) = alt((map(selector_combinator, |_| ()), function_argument))(input)?;
        let (input, _) = tag_rparen(input)?;
        return Ok((input, ident[1..].to_string()));
    }

    Ok((input, ident[1..].to_string()))
}

fn pseudo_class(input: Tokens) -> IResult<Tokens, BasicSelector> {
    let _a = InputDebug::print(&input, "pseudo_class");
    let (input, result) = many1(pseudo_class_single)(input)?;
    Ok((
        input,
        BasicSelector::PseudoClass(PseudoClassSelector {
            0: result[0].clone(),
        }),
    ))
}

fn pseudo_element(input: Tokens) -> IResult<Tokens, BasicSelector> {
    let (input, _) = tag_double_colon(input)?;
    let (input, ident) = identifier(input)?;

    Ok((
        input,
        BasicSelector::PseudoElement(PseudoElementSelector {
            0: ident[2..].to_string(),
        }),
    ))
}

fn universal_selector(input: Tokens) -> IResult<Tokens, BasicSelector> {
    let (input, _) = tag_star(input)?;

    Ok((
        input,
        BasicSelector::Universal(IdSelector { 0: "".to_string() }),
    ))
}

fn class_selector(input: Tokens) -> IResult<Tokens, BasicSelector> {
    let (input, _) = tag_dot(input)?;
    let (input, identifier) = identifier(input)?;

    Ok((input, BasicSelector::Class(ClassSelector { 0: identifier })))
}

fn type_selector(input: Tokens) -> IResult<Tokens, BasicSelector> {
    let (input, identifier) = identifier(input)?;
    Ok((input, BasicSelector::Type(TypeSelector { 0: identifier })))
}

fn id_selector(input: Tokens) -> IResult<Tokens, BasicSelector> {
    let (input, _) = tag_hash(input)?;
    let (input, identifier) = identifier(input)?;

    Ok((
        input,
        BasicSelector::Id(IdSelector {
            0: identifier.to_string(),
        }),
    ))
}

fn attribute_selector(input: Tokens) -> IResult<Tokens, BasicSelector> {
    let (input, _) = tag_lbracket(input)?;
    let (input, _) = assign_expression(input)?;
    let (input, _) = tag_rbracket(input)?;

    Ok((
        input,
        BasicSelector::Attribute(AttributeSelector { 0: "".to_string() }),
    ))
}

fn basic_selector(input: Tokens) -> IResult<Tokens, BasicSelector> {
    let _a = InputDebug::print(&input, "basic selector");
    let (input, selector) = alt((
        universal_selector,
        class_selector,
        id_selector,
        type_selector,
        attribute_selector,
        pseudo_class,
        pseudo_element,
    ))(input)?;

    Ok((input, selector))
}

fn child_combinator(input: Tokens) -> IResult<Tokens, SelectorCombinator> {
    let (input, basic_selector) = basic_selector(input)?;
    let (input, _) = tag_gt(input)?;
    let (input, combinator) = selector_combinator(input)?;

    Ok((
        input,
        SelectorCombinator::Child(basic_selector, Box::new(combinator)),
    ))
}

fn general_sibling_combinator(input: Tokens) -> IResult<Tokens, SelectorCombinator> {
    let (input, basic_selector) = basic_selector(input)?;
    let (input, _) = tag_tilde(input)?;
    let (input, combinator) = selector_combinator(input)?;

    Ok((
        input,
        SelectorCombinator::GeneralSibling(basic_selector, Box::new(combinator)),
    ))
}

fn adjacent_sibling_combinator(input: Tokens) -> IResult<Tokens, SelectorCombinator> {
    let (input, basic_selector) = basic_selector(input)?;
    let (input, _) = tag_plus(input)?;
    let (input, combinator) = selector_combinator(input)?;

    Ok((
        input,
        SelectorCombinator::AdjacentSibling(basic_selector, Box::new(combinator)),
    ))
}

fn descendant_sibling_combinator(input: Tokens) -> IResult<Tokens, SelectorCombinator> {
    let _a = InputDebug::print(&input, "descendant_sibling_combinator");
    let (input, basic_selector) = basic_selector(input)?;
    let (input, combinator) = selector_combinator(input)?;

    Ok((
        input,
        SelectorCombinator::Descendant(basic_selector, Box::new(combinator)),
    ))
}

fn selector_combinator(input: Tokens) -> IResult<Tokens, SelectorCombinator> {
    let _a = InputDebug::print(&input, "selector_combinator");
    alt((
        child_combinator,
        general_sibling_combinator,
        adjacent_sibling_combinator,
        descendant_sibling_combinator,
        map(basic_selector, |s| SelectorCombinator::Basic(s)),
    ))(input)
}

fn selector_list(input: Tokens) -> IResult<Tokens, Vec<SelectorCombinator>> {
    separated_list1(tag_comma, selector_combinator)(input)
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

    fn conditional_at_rule_media(input: Tokens) -> IResult<Tokens, ()> {
        let (mut input, _) = tag_ident("@media", input)?;
        loop {
            let token = input.first();
            if Some(&Token::LeftBrace) == token {
                let ret = take(1usize)(input)?;
                input = ret.0;
                break;
            }

            let ret = take(1usize)(input)?;
            input = ret.0;
        }

        let (input, _) = many0(Self::parse_block)(input)?;
        let (input, _) = tag_rbrace(input)?;

        Ok((input, ()))
    }

    fn conditional_at_rule(input: Tokens) -> IResult<Tokens, Option<StyleBlock>> {
        let (input, _) = Self::conditional_at_rule_media(input)?;
        Ok((input, None))
    }

    fn parse_block(input: Tokens) -> IResult<Tokens, Option<StyleBlock>> {
        let (input, selectors) = selector_list(input)?;
        let (input, _) = tag_lbrace(input)?;
        let (input, style) = style_inline(input)?;
        let (input, _) = tag_rbrace(input)?;

        let _a = InputDebug::print(&input, "parse block completed");
        Ok((input, Some(StyleBlock { selectors, style })))
    }

    fn style<'a>(&self, tokens: Tokens<'a>) -> IResult<Tokens<'a>, Vec<StyleBlock>> {
        let (input, ret) = many0(alt((
            Self::conditional_at_rule,
            self.bind(Self::regular_at_rule),
            Self::parse_block,
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
