#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlockId(pub usize);

impl core::fmt::Display for BlockId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "${}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(pub usize);

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "v{}", self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add,
    And,
    Eq,
    Lt,
    Le,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    BinOp(Value, BinaryOp, Value, Value),
    Move(Value, Value),
    Const(Value, u64),
    LoadDataAddr(Value, usize),
    Syscall(Option<Value>, Vec<Value>),
}

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

#[derive(Clone)]
pub struct BasicBlock {
    pub params: Vec<Value>,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

pub struct Program {
    pub blocks: Vec<BasicBlock>,
    pub data: Vec<u8>,
}

const SYS_WRITE: u64 = 64;
const SYS_EXIT: u64 = 93;
const STDOUT: u64 = 1;

impl Program {
    pub fn hello_world() -> Self {
        let v0 = Value(0);
        let v1 = Value(1);
        let v2 = Value(2);
        let v3 = Value(3);

        let block = BasicBlock {
            params: vec![],
            instructions: vec![
                Instruction::Const(v0, STDOUT),
                Instruction::LoadDataAddr(v1, 0),
                Instruction::Const(v2, 12),
                Instruction::Const(v3, SYS_WRITE),
                Instruction::Syscall(None, vec![v3, v0, v1, v2]),
                Instruction::Const(v0, 0),
                Instruction::Const(v3, SYS_EXIT),
                Instruction::Syscall(None, vec![v3, v0]),
            ],
            terminator: Terminator::ReturnVoid,
        };

        Program {
            blocks: vec![block],
            data: b"Hello World\n".to_vec(),
        }
    }
}
