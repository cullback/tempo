use pest::Span;

// AST Node types
#[derive(Debug)]
pub enum AstNode<'a> {
    Program(Program<'a>),
    Assignment(Assignment<'a>),
    Expression(Expression<'a>),
    Identifier(Identifier<'a>),
    Number(Number<'a>),
    FunctionCall(FunctionCall<'a>),
    FunctionDefinition(FunctionDefinition<'a>),
    Block(Block<'a>),
}

#[derive(Debug)]
pub struct Program<'a> {
    pub assignments: Vec<Assignment<'a>>,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub struct Assignment<'a> {
    pub identifier: Identifier<'a>,
    pub expression: Box<Expression<'a>>,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub struct Identifier<'a> {
    pub name: String,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub struct Number<'a> {
    pub value: i64,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub enum Expression<'a> {
    Number(Number<'a>),
    Identifier(Identifier<'a>),
    FunctionCall(FunctionCall<'a>),
    FunctionDefinition(FunctionDefinition<'a>),
    Block(Block<'a>),
}

#[derive(Debug)]
pub struct FunctionCall<'a> {
    pub function_name: Identifier<'a>,
    pub arguments: Vec<Expression<'a>>,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub struct FunctionDefinition<'a> {
    pub parameters: Vec<Identifier<'a>>,
    pub body: Box<Expression<'a>>,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub assignments: Vec<Assignment<'a>>,
    pub expression: Box<Expression<'a>>,
    pub span: Span<'a>,
}
