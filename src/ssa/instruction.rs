/// A reference to a value in an IR function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(pub usize);

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "v{}", self.0)
    }
}

/// Operations that instructions can perform.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Eq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
}

/// An instruction that exists in a basic block.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    BinOp(Value, BinaryOp, Value, Value),
    Move(Value, Value),
    Const(Value, u64),
    LoadDataAddr(Value, usize),
    Syscall(Option<Value>, Vec<Value>),
}
