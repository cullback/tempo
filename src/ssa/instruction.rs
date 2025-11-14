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
    Call(Value, String, Vec<Value>),
}

impl core::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::And => write!(f, "&&"),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::NotEq => write!(f, "!="),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::Le => write!(f, "<="),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::Ge => write!(f, ">="),
        }
    }
}

impl core::fmt::Display for Instruction {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Instruction::BinOp(dest, op, lhs, rhs) => {
                write!(f, "{} = {} {} {}", dest, lhs, op, rhs)
            }
            Instruction::Move(dest, src) => {
                write!(f, "{} = {}", dest, src)
            }
            Instruction::Const(dest, val) => {
                write!(f, "{} = {}", dest, val)
            }
            Instruction::LoadDataAddr(dest, offset) => {
                write!(f, "{} = &data[{}]", dest, offset)
            }
            Instruction::Syscall(result, args) => {
                if let Some(result) = result {
                    write!(f, "{} = ", result)?;
                }
                write!(f, "syscall(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Instruction::Call(result, name, args) => {
                write!(f, "{} = {}(", result, name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}
