
#[derive(Debug)]
pub enum Literal {
    NullLiteral,
    BooleanLiteral(bool),
    StringLiteral(String),
}

#[derive(Debug)]
pub enum PrimaryExpression {
    This,
    IdentifierReference(String),
    Literal(Literal),
}

#[derive(Debug)]
pub enum MemberExpression {
    PrimaryExpression(PrimaryExpression),
    MemberExpressionDotIdentiferName(Box<MemberExpression>, String),
}

#[derive(Debug)]
pub struct ArgumentList(pub Vec<Box<Expression>>);

#[derive(Debug)]
pub enum Expression {
    AssignmentExpression,
    ConditionalExpression,
    ShortCircuitExpression,
    LogicalORExpression,
    LogicalANDExpression,
    BitwiseORExpression,
    BitwiseXORExpression,
    BitwiseANDExpression,
    EqualityExpression,
    RelationalExpression,
    ShiftExpression,
    AdditiveExpression,
    MultiplicativeExpression,
    ExponentiationExpression,
    UnaryExpression,
    UpdateExpression,
    LeftHandSideExpression,
    MemberExpression(MemberExpression),
    NewExpression,
    CallExpression,
    CoverCallExpressionAndAsyncArrowHead(MemberExpression, ArgumentList),
}

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Vec<Box<Expression>>),
}


#[derive(Debug)]
pub enum ScriptBody {
    StatementList(Vec<Box<Statement>>),
}

#[derive(Debug)]
pub enum Script {
    ScriptBody(Option<ScriptBody>),
}

pub fn concat_str<T: Into<String>, U: Into<String>>(a: T, b: U) -> String {
    format!("{}{}", a.into(), b.into())
}

pub fn concat_str4(a: &str, b: &str, c: &str, d: &str) -> String {
    format!("{}{}{}{}", a, b, c, d)
}

pub fn append<T>(a: Vec<T>, b: T) -> Vec<T> {
    let mut x = a;
    x.extend(vec![b]);
    x
}
