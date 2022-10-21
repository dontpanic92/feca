use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_while_m_n},
    character::complete::{alpha1, alphanumeric1, hex_digit1, multispace0, one_of},
    combinator::{map, opt, peek, recognize},
    multi::{many0_count, many1, separated_list0, separated_list1},
    number::complete::double,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use parser_utils::{boxed, make_vec, to_string, w, w2};

use crate::ast::{
    ArgumentList, AssignmentExpression, BindingElement, CallExpression, Declaration, Expression,
    FormalParameter, FormalParameters, FunctionBody, FunctionDeclaration, HoistableDeclaration,
    LeftHandSideExpression, Literal, MemberExpression, PrimaryExpression, Script, ScriptBody,
    SingleNameBinding, Statement, StatementListItem,
};

pub fn parse(input: &str) -> IResult<&str, Script> {
    script(input)
}

macro_rules! def {
    ($name: ident: $ret_ty: ty { $($var: ident = $parser: expr;)* } $ret: expr;) => {
        fn $name(input: &str) -> IResult<&str, $ret_ty> {
            // println!("entering {}", stringify!($name));
            $( let (input, _) = multispace0(input)?; let (input, $var) = $parser(input)?; )*

            // println!("leaving {}: {}", stringify!($name), input);
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

// Lexical Grammar

p!(
    private_identifier,
    String,
    preceded(tag("#"), identifier_name)
);

// TODO
p!(
    identifier_name,
    String,
    to_string(w(recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_"))))
    ))))
);

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
    map(w(tag("null")), |_| Literal::NullLiteral)
);

p!(
    boolean_literal,
    Literal,
    alt((
        map(w(tag("true")), |_| Literal::BooleanLiteral(true)),
        map(w(tag("false")), |_| Literal::BooleanLiteral(false)),
    )),
);

p!(
    string_literal,
    Literal,
    map(
        alt((
            w(delimited(tag("\""), double_string_characters, tag("\""))),
            w(delimited(tag("'"), single_string_characters, tag("'"))),
        )),
        |s| Literal::StringLiteral(s)
    ),
);

p!(
    number_literal,
    Literal,
    map(double, |n| Literal::NumberLiteral(n)),
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

// 13 Expressions

// 13.1 Identifiers

p!(identifier_reference, String, identifier);

p!(binding_identifier, String, identifier);

p!(identifier, String, identifier_name);

// 13.2 Primary Expression

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
    alt((
        null_literal,
        boolean_literal,
        string_literal,
        number_literal
    ))
);

p!(
    initializer,
    Expression,
    preceded(tag("="), assignment_expression)
);

fn member_expression(input: &str) -> IResult<&str, MemberExpression> {
    let (mut input, mut member_expr) = alt((map(primary_expression, |p| {
        MemberExpression::PrimaryExpression(p)
    }),))(input)?;

    loop {
        let next: IResult<&str, &str> = peek(w(tag(".")))(input);
        if next.is_ok() {
            let (i, _) = w(tag("."))(input)?;
            let (i, name) = alt((private_identifier, identifier_name))(i)?;

            input = i;
            member_expr =
                MemberExpression::MemberExpressionDotIdentiferName(Box::new(member_expr), name);
        } else {
            return Ok((input, member_expr));
        }
    }
}

p!(
    call_expression,
    CallExpression,
    cover_call_expression_and_async_arrow_head
);

p2!(
    arguments,
    ArgumentList,
    alt((
        make_vec(pair(w(tag("(")), w(tag(")")))),
        // delimited(w(tag("(")), argument_list, w(tag(")"))),
        delimited(
            w(tag("(")),
            terminated(argument_list, opt(tag(","))),
            w(tag(")"))
        ),
    )),
    ArgumentList
);

p!(
    argument_list,
    Vec<Box<Expression>>,
    separated_list1(w2(tag(",")), boxed(assignment_expression))
);

p2!(
    new_expression,
    LeftHandSideExpression,
    member_expression,
    LeftHandSideExpression::MemberExpression
);

p!(
    left_hand_side_expression,
    LeftHandSideExpression,
    alt((
        map(call_expression, |e| LeftHandSideExpression::CallExpression(
            e
        )),
        new_expression,
    ))
);

p!(
    update_expression,
    Expression,
    map(left_hand_side_expression, |e| {
        Expression::LeftHandSideExpression(e)
    })
);

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

p!(conditional_exprsesion, Expression, short_circuit_expression);

// p!(assignment_expression, Expression, conditional_exprsesion);

fn assignment_expression(input: &str) -> IResult<&str, Expression> {
    let (input, expr) = conditional_exprsesion(input)?;
    if let Expression::LeftHandSideExpression(lhs_expr) = &expr {
        let res: IResult<&str, &str> = peek(w(tag("=")))(input);
        if let Ok((input, next)) = res {
            match next {
                "=" => {
                    let (input, op) = w(tag("="))(input)?;
                    let (input, expr) = assignment_expression(input)?;
                    return Ok((
                        input,
                        Expression::AssignmentExpression(AssignmentExpression {
                            lhs_expr: lhs_expr.clone(),
                            op: op.to_string(),
                            expr: Box::new(expr),
                        }),
                    ));
                }
                _ => {}
            }
        }
    }

    Ok((input, expr))
}

def! {
    expression: Vec<Box<Expression>> {
        expr = separated_list1(tag(","), boxed(assignment_expression));
    } expr;
}

// 14 Statements and Declarations

def! {
    statement: Statement {
        expr_stmt = expression_statement;
     } Statement::ExpressionStatement(expr_stmt);
}

p!(
    declaration,
    Declaration,
    map(
        hoistable_declaration,
        |d| Declaration::HoistableDeclaration(d)
    )
);

p!(
    hoistable_declaration,
    HoistableDeclaration,
    map(function_declaration, |d| {
        HoistableDeclaration::FunctionDeclaration(d)
    })
);

// 14.2 Block

def! {
    statement_list: Vec<Box<StatementListItem>> {
        stmt_list = many1(boxed(statement_list_item));
     } stmt_list;
}

def! {
    statement_list_item: StatementListItem {
        stmt = alt(( map(declaration, |d| StatementListItem::Declaration(d)), map(statement, |s| StatementListItem::Statement(s)),));
    } stmt;
}

// 14.3 Decalrations and Variable Statement

p!(
    binding_element,
    BindingElement,
    map(single_name_binding, |b| BindingElement::SingleNameBinding(
        b
    ))
);

p!(
    single_name_binding,
    SingleNameBinding,
    map(pair(binding_identifier, opt(initializer)), |p| {
        SingleNameBinding {
            name: p.0,
            initializer: p.1,
        }
    })
);

p!(
    expression_statement,
    Vec<Box<Expression>>,
    terminated(expression, opt(w(tag(";"))))
);

// 15 Functions and Classes

// 15.1 Parameter Lists

p!(
    formal_parameters,
    FormalParameters,
    map(separated_list0(tag(","), boxed(formal_parameter)), |p| {
        FormalParameters::FormalParameterList(p)
    })
);

p!(
    formal_parameter,
    FormalParameter,
    map(binding_element, |e| FormalParameter::BindingElement(e)),
);

// 15.2 Function Definitions

p!(
    function_declaration,
    FunctionDeclaration,
    map(
        tuple((
            w(tag("function")),
            opt(binding_identifier),
            w(tag("(")),
            formal_parameters,
            w(tag(")")),
            w(tag("{")),
            function_body,
            w(tag("}"))
        )),
        |t| FunctionDeclaration {
            name: t.1,
            parameters: t.3,
            body: t.6
        }
    )
);

p!(
    function_body,
    FunctionBody,
    map(opt(statement_list), |s| FunctionBody(s))
);

p!(
    cover_call_expression_and_async_arrow_head,
    CallExpression,
    map(pair(member_expression, arguments), |p| {
        CallExpression::CoverCallExpressionAndAsyncArrowHead(p.0, p.1)
    })
);

// Script and Modules

def! {
    script: Script {
        script_body = w2(script_body);
    } Script::ScriptBody(Some(script_body));
}

def! {
    script_body: ScriptBody {
        stmt_list = statement_list;
    } ScriptBody::StatementList(stmt_list);
}
