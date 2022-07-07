use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_while_m_n},
    character::complete::{alphanumeric1, hex_digit1, multispace0, one_of},
    combinator::{map, opt},
    error::ParseError,
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

use crate::ast::{
    ArgumentList, Expression, Literal, MemberExpression, PrimaryExpression, Script, ScriptBody,
    Statement,
};

pub fn parse(input: &str) -> IResult<&str, Script> {
    script(input)
}

/*macro_rules! p {
    ($output: ident, $parser: expr, $input: ident) => {
        let ($input, $output) = $parser($input)?;
        let ($input, _) = multispace0($input)?;
    };
}

macro_rules! ok {
    ($input: ident, $output: expr) => {
        return Ok(($input, $output));
    };
}*/

macro_rules! def {
    ($name: ident: $ret_ty: ty { $($var: ident = $parser: expr;)* } $ret: expr;) => {
        fn $name(input: &str) -> IResult<&str, $ret_ty> {
            println!("entering {}: {}", stringify!($name), input);
            $( let (input, _) = multispace0(input)?; let (input, $var) = $parser(input)?; )*

            println!("leaving {}: {}", stringify!($name), input);
            return Ok((input, $ret));
        }
    };
}

macro_rules! p {
    ($name: ident, $ret_ty: ty, $parser: expr $(,)?) => {
        def! {
            $name: $ret_ty {
                ret = $parser;
            } ret;
        }
    };
}

macro_rules! p2 {
    ($name: ident, $ret_ty: ty, $parser: expr, $ret: expr) => {
        def! {
            $name: $ret_ty {
                val = $parser;
            } $ret(val);
        }
    };
}

fn boxed<'a, F: 'a, O, I, E: ParseError<I>>(inner: F) -> impl FnMut(I) -> IResult<I, Box<O>, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
{
    map(inner, |x| Box::new(x))
}

fn make_vec<'a, F: 'a, O, U, I, E: ParseError<I>>(
    inner: F,
) -> impl FnMut(I) -> IResult<I, Vec<U>, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
{
    map(inner, |x| vec![])
}

fn to_string<'a, F: 'a, O, I, E: ParseError<I>>(inner: F) -> impl FnMut(I) -> IResult<I, String, E>
where
    O: Into<String>,
    F: FnMut(I) -> IResult<I, O, E>,
{
    map(inner, |x| x.into())
}

// Lexical Grammar

p!(
    private_identifier,
    String,
    preceded(tag("#"), identifier_name)
);

// TODO
p!(identifier_name, String, to_string(alphanumeric1));

p!(
    unicode_id_start,
    String,
    to_string(take_while_m_n(1, 1, |ch: char| ch.is_alphanumeric()))
);

p!(
    unicode_id_continue,
    String,
    to_string(take_while_m_n(1, 1, |ch: char| ch.is_alphanumeric()))
);

p!(
    null_literal,
    Literal,
    map(tag("null"), |_| Literal::NullLiteral)
);

p!(
    boolean_literal,
    Literal,
    alt((
        map(tag("true"), |_| Literal::BooleanLiteral(true)),
        map(tag("false"), |_| Literal::BooleanLiteral(false)),
    )),
);

p!(
    string_literal,
    Literal,
    map(
        alt((
            delimited(tag("\""), double_string_characters, tag("\"")),
            delimited(tag("'"), single_string_characters, tag("'")),
        )),
        |s| Literal::StringLiteral(s)
    ),
);

p!(
    single_string_characters,
    String,
    to_string(escaped(is_not("'"), '\\', one_of("'")))
);

p!(
    double_string_characters,
    String,
    to_string(escaped(is_not("\""), '\\', one_of("\""))),
);

p!(
    hex_4_digits,
    String,
    to_string(take_while_m_n(4, 4, |ch: char| ch.is_ascii_hexdigit())),
);

p!(hex_digits, String, to_string(hex_digit1));

p!(code_point, String, hex_digits);

// Expressions

p!(identifier_reference, String, identifier);

p!(identifier, String, identifier_name);

p!(
    primary_expression,
    PrimaryExpression,
    alt((
        map(tag("this"), |_| PrimaryExpression::This),
        map(identifier_reference, |ident| {
            PrimaryExpression::IdentifierReference(ident)
        }),
        map(literal, |l| PrimaryExpression::Literal(l)),
    ))
);

p!(
    literal,
    Literal,
    alt((null_literal, boolean_literal, string_literal,))
);

p!(
    member_expression,
    MemberExpression,
    alt((
        map(primary_expression, |p| MemberExpression::PrimaryExpression(
            p
        )),
        map(
            separated_pair(member_expression, tag("."), identifier_name),
            |p| MemberExpression::MemberExpressionDotIdentiferName(Box::new(p.0), p.1)
        ),
    ))
);

p!(
    call_expression,
    Expression,
    cover_call_expression_and_async_arrow_head
);

p2!(
    arguments,
    ArgumentList,
    alt((
        make_vec(pair(tag("("), tag(")"))),
        delimited(tag("("), argument_list, tag(")")),
        delimited(tag("("), terminated(argument_list, tag(",")), tag(")")),
    )),
    ArgumentList
);

p!(
    argument_list,
    Vec<Box<Expression>>,
    many1(boxed(assignment_expression))
);

p2!(
    new_expression,
    Expression,
    member_expression,
    Expression::MemberExpression
);

p!(
    left_hand_side_expression,
    Expression,
    alt((new_expression, call_expression))
);

p!(update_expression, Expression, left_hand_side_expression);

p!(unary_expression, Expression, update_expression);

p!(exponentiation_expression, Expression, unary_expression);

p!(
    multiplicative_expression,
    Expression,
    exponentiation_expression
);

p!(additive_expression, Expression, multiplicative_expression);

p!(shift_expression, Expression, additive_expression);

p!(relational_expression, Expression, shift_expression);

p!(equality_expression, Expression, relational_expression);

p!(bitwise_and_expression, Expression, equality_expression);

p!(bitwise_xor_expression, Expression, bitwise_and_expression);

p!(bitwise_or_expression, Expression, bitwise_xor_expression);

p!(logical_and_expression, Expression, bitwise_or_expression);

p!(logical_or_expression, Expression, logical_and_expression);

p!(short_circuit_expression, Expression, logical_or_expression);

def! {
    conditional_exprsesion: Expression {
        expr = short_circuit_expression;
    } expr;
}

def! {
    assignment_expression: Expression {
        expr = conditional_exprsesion;
    } expr;
}

def! {
    expression: Vec<Box<Expression>> {
        expr = separated_list1(tag(","), boxed(assignment_expression));
    } expr;
}

// Statements and Declarations

def! {
    statement: Box<Statement> {
        expr_stmt = expression_statement;
     } Box::new(Statement::ExpressionStatement(expr_stmt));
}

def! {
    statement_list: Vec<Box<Statement>> {
        stmt_list = many1(statement_item);
     } stmt_list;
}

def! {
    statement_item: Box<Statement>{
        stmt = statement;
    } stmt;
}

def! {
    expression_statement: Vec<Box<Expression>> {
        expr = terminated(expression, tag(";"));
    } expr;
}

// Async Function Definitions

p!(
    cover_call_expression_and_async_arrow_head,
    Expression,
    map(pair(member_expression, arguments), |p| {
        Expression::CoverCallExpressionAndAsyncArrowHead(p.0, p.1)
    })
);

// Script and Modules

def! {
    script: Script {
        script_body = script_body;
    } Script::ScriptBody(Some(script_body));
}

def! {
    script_body: ScriptBody {
        stmt_list = statement_list;
    } ScriptBody::StatementList(stmt_list);
}
