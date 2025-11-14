#[derive(Debug)]
pub enum Instruction {
    MovImm {
        dest: Register,
        value: u64,
    },
    AdrPcRel {
        dest: Register,
        offset: i32,
    },
    Syscall,
    Add {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Sub {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Mul {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Div {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    Label {
        name: String,
    },
    Branch {
        condition: Register,
        target: String,
    },
    Jump {
        target: String,
    },
    Mov {
        dest: Register,
        src: Register,
    },
    Cmp {
        src1: Register,
        src2: Register,
    },
    CSet {
        dest: Register,
        condition: Condition,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
}

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub data: Vec<u8>,
}
