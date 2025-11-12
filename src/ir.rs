pub enum Instruction {
    MovImm { dest: Register, value: u64 },
    AdrPcRel { dest: Register, offset: i32 },
    Syscall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    X0,
    X1,
    X2,
    X8,
}

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub data: Vec<u8>,
}
