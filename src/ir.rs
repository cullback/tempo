pub enum Instruction {
    MovImm { dest: Register, value: u64 },
    AdrPcRel { dest: Register, offset: i32 },
    Syscall,
}

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

impl Program {
    pub fn hello_world() -> Self {
        Program {
            instructions: vec![
                Instruction::MovImm {
                    dest: Register::X0,
                    value: 1,
                },
                Instruction::AdrPcRel {
                    dest: Register::X1,
                    offset: 28,
                },
                Instruction::MovImm {
                    dest: Register::X2,
                    value: 12,
                },
                Instruction::MovImm {
                    dest: Register::X8,
                    value: 64,
                },
                Instruction::Syscall,
                Instruction::MovImm {
                    dest: Register::X0,
                    value: 0,
                },
                Instruction::MovImm {
                    dest: Register::X8,
                    value: 93,
                },
                Instruction::Syscall,
            ],
            data: b"Hello World\n".to_vec(),
        }
    }
}
