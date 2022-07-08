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
pub enum Declaration {
    HoistableDeclaration(HoistableDeclaration),
    ClassDeclaration,
    LexicalDeclaration,
}

#[derive(Debug)]
pub enum HoistableDeclaration {
    FunctionDeclaration(FunctionDeclaration),
}


#[derive(Debug)]
pub struct SingleNameBinding {
    pub name: String,
    pub initializer: Option<Expression>,
}

#[derive(Debug)]
pub enum BindingElement {
    SingleNameBinding(SingleNameBinding),
    BindingPattern,
}

#[derive(Debug)]
pub enum FormalParameter {
    BindingElement(BindingElement),
}

pub type FormalParameterList = Vec<Box<FormalParameter>>;

#[derive(Debug)]
pub enum FormalParameters {
    FormalParameterList(FormalParameterList),
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: Option<String>,
    pub parameters: FormalParameters,
    pub body: FunctionBody,
}

#[derive(Debug)]
pub struct FunctionBody(pub Option<StatementList>);

#[derive(Debug)]
pub enum StatementListItem {
    Statement(Statement),
    Declaration(Declaration),
}

pub type StatementList = Vec<Box<StatementListItem>>;

#[derive(Debug)]
pub enum ScriptBody {
    StatementList(StatementList),
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
