

#[derive(Debug, Clone)]
pub enum Literal {
    NullLiteral,
    BooleanLiteral(bool),
    StringLiteral(String),
    NumberLiteral(f64),
}

#[derive(Debug, Clone)]
pub enum PrimaryExpression {
    This,
    IdentifierReference(String),
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub enum MemberExpression {
    PrimaryExpression(PrimaryExpression),
    MemberExpressionDotIdentiferName(Box<MemberExpression>, String),
}

#[derive(Debug, Clone)]
pub struct ArgumentList(pub Vec<Box<Expression>>);

#[derive(Debug, Clone)]
pub enum CallExpression {
    CoverCallExpressionAndAsyncArrowHead(MemberExpression, ArgumentList),
}

#[derive(Debug, Clone)]
pub enum LeftHandSideExpression {
    MemberExpression(MemberExpression),
    NewExpression,
    CallExpression(CallExpression),
}

#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub lhs_expr: LeftHandSideExpression,
    pub op: String,
    pub expr: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    AssignmentExpression(AssignmentExpression),
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
    LeftHandSideExpression(LeftHandSideExpression),
}

#[derive(Debug, Clone)]
pub enum Statement {
    ExpressionStatement(Vec<Box<Expression>>),
}

#[derive(Debug, Clone)]
pub enum Declaration {
    HoistableDeclaration(HoistableDeclaration),
    ClassDeclaration,
    LexicalDeclaration,
}

#[derive(Debug, Clone)]
pub enum HoistableDeclaration {
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug, Clone)]
pub struct SingleNameBinding {
    pub name: String,
    pub initializer: Option<Expression>,
}

#[derive(Debug, Clone)]
pub enum BindingElement {
    SingleNameBinding(SingleNameBinding),
    BindingPattern,
}

#[derive(Debug, Clone)]
pub enum FormalParameter {
    BindingElement(BindingElement),
}

pub type FormalParameterList = Vec<Box<FormalParameter>>;

#[derive(Debug, Clone)]
pub enum FormalParameters {
    FormalParameterList(FormalParameterList),
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: Option<String>,
    pub parameters: FormalParameters,
    pub body: FunctionBody,
}

#[derive(Debug, Clone)]
pub struct FunctionBody(pub Option<StatementList>);

#[derive(Debug, Clone)]
pub enum StatementListItem {
    Statement(Statement),
    Declaration(Declaration),
}

pub type StatementList = Vec<Box<StatementListItem>>;

#[derive(Debug, Clone)]
pub enum ScriptBody {
    StatementList(StatementList),
}

#[derive(Debug, Clone)]
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
