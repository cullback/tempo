use crate::ssa::instruction::{Instruction, Value};

/// [`BlockId`] represents a reference to a basic block in an IR function.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlockId(pub usize);

impl core::fmt::Display for BlockId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "b{}", self.0)
    }
}

/// [`Terminator`] terminates a given basic block.
#[derive(Debug, Clone)]
pub enum Terminator {
    None,
    ReturnVoid,
    Return(Value),
    Jump(BlockId, Vec<Value>),
    Branch(Value, BlockId, Vec<Value>, BlockId, Vec<Value>),
}

impl core::fmt::Display for Terminator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::None => write!(f, "noterm"),
            Self::ReturnVoid => write!(f, "ret void"),
            Self::Return(v) => write!(f, "ret {v}"),
            Self::Jump(b, args) => {
                write!(f, "jump {b}")?;
                if !args.is_empty() {
                    write!(f, "(")?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{arg}")?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
            Self::Branch(c, t, t_args, e, e_args) => {
                write!(f, "branch {c}, {t}(")?;
                for (i, arg) in t_args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{arg}")?;
                }
                write!(f, "), {e}(")?;
                for (i, arg) in e_args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{arg}")?;
                }
                write!(f, ")")
            }
        }
    }
}

/// A basic block is a sequence of instructions that ends with a terminator.
#[derive(Clone, Debug)]
pub struct BasicBlock {
    pub params: Vec<Value>,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

impl core::fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if !self.params.is_empty() {
            write!(f, "params: ")?;
            for (i, param) in self.params.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", param)?;
            }
            writeln!(f)?;
        }

        for instr in &self.instructions {
            writeln!(f, "{}", instr)?;
        }

        write!(f, "{}", self.terminator)?;

        Ok(())
    }
}
