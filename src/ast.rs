use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Span<'a> {
    pub source: &'a str,
}

impl<'a> Span<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }
}

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

impl fmt::Display for Program<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for assignment in &self.assignments {
            writeln!(f, "{}", assignment)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Assignment<'a> {
    pub identifier: Identifier<'a>,
    pub expression: Box<Expression<'a>>,
    pub span: Span<'a>,
}

impl fmt::Display for Assignment<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.identifier, self.expression)
    }
}

#[derive(Debug)]
pub struct Identifier<'a> {
    pub name: String,
    pub span: Span<'a>,
}

impl fmt::Display for Identifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct Number<'a> {
    pub value: i64,
    pub span: Span<'a>,
}

impl fmt::Display for Number<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub enum Expression<'a> {
    Number(Number<'a>),
    Identifier(Identifier<'a>),
    FunctionCall(FunctionCall<'a>),
    FunctionDefinition(FunctionDefinition<'a>),
    Block(Block<'a>),
}

impl fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Identifier(i) => write!(f, "{}", i),
            Expression::FunctionCall(fc) => write!(f, "{}", fc),
            Expression::FunctionDefinition(fd) => write!(f, "{}", fd),
            Expression::Block(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug)]
pub struct FunctionCall<'a> {
    pub function_name: Identifier<'a>,
    pub arguments: Vec<Expression<'a>>,
    pub span: Span<'a>,
}

impl fmt::Display for FunctionCall<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.function_name)?;
        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ")")
    }
}

#[derive(Debug)]
pub struct FunctionDefinition<'a> {
    pub parameters: Vec<Identifier<'a>>,
    pub body: Box<Expression<'a>>,
    pub span: Span<'a>,
}

impl fmt::Display for FunctionDefinition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn(")?;
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param)?;
        }
        write!(f, ") {{ {} }}", self.body)
    }
}

#[derive(Debug)]
pub struct Block<'a> {
    pub assignments: Vec<Assignment<'a>>,
    pub expression: Box<Expression<'a>>,
    pub span: Span<'a>,
}

impl fmt::Display for Block<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for assignment in &self.assignments {
            writeln!(f, "  {}", assignment)?;
        }
        writeln!(f, "  {}", self.expression)?;
        write!(f, "}}")
    }
}
