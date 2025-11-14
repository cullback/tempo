use std::fmt;

#[derive(Debug, Clone, Copy)]
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
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
}

impl Register {
    fn num(&self) -> u32 {
        match self {
            Register::X0 => 0,
            Register::X1 => 1,
            Register::X2 => 2,
            Register::X3 => 3,
            Register::X4 => 4,
            Register::X5 => 5,
            Register::X6 => 6,
            Register::X7 => 7,
            Register::X8 => 8,
            Register::X9 => 9,
            Register::X10 => 10,
            Register::X11 => 11,
            Register::X12 => 12,
            Register::X13 => 13,
            Register::X14 => 14,
            Register::X15 => 15,
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::X0 => write!(f, "x0"),
            Register::X1 => write!(f, "x1"),
            Register::X2 => write!(f, "x2"),
            Register::X3 => write!(f, "x3"),
            Register::X4 => write!(f, "x4"),
            Register::X5 => write!(f, "x5"),
            Register::X6 => write!(f, "x6"),
            Register::X7 => write!(f, "x7"),
            Register::X8 => write!(f, "x8"),
            Register::X9 => write!(f, "x9"),
            Register::X10 => write!(f, "x10"),
            Register::X11 => write!(f, "x11"),
            Register::X12 => write!(f, "x12"),
            Register::X13 => write!(f, "x13"),
            Register::X14 => write!(f, "x14"),
            Register::X15 => write!(f, "x15"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    MovImm { rd: Register, imm: u16 },
    Adr { rd: Register, offset: i32 },
    Svc { imm: u16 },
}

impl Instruction {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Instruction::MovImm { rd, imm } => {
                // MOVZ rd, #imm (move wide with zero)
                let instr = 0xd2800000u32 | ((*imm as u32) << 5) | rd.num();
                instr.to_le_bytes().to_vec()
            }
            Instruction::Adr { rd, offset } => {
                // ADR rd, offset
                let immlo = (*offset & 0x3) as u32;
                let immhi = ((*offset >> 2) & 0x7FFFF) as u32;
                let instr =
                    0x10000000u32 | (immlo << 29) | (immhi << 5) | rd.num();
                instr.to_le_bytes().to_vec()
            }
            Instruction::Svc { imm } => {
                // SVC #imm
                let instr = 0xd4000001u32 | ((*imm as u32) << 5);
                instr.to_le_bytes().to_vec()
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::MovImm { rd, imm } => {
                write!(f, "mov {}, #{}", rd, imm)
            }
            Instruction::Adr { rd, offset } => {
                write!(f, "adr {}, #{}", rd, offset)
            }
            Instruction::Svc { imm } => write!(f, "svc #{}", imm),
        }
    }
}

pub fn encode_instructions(instructions: &[Instruction]) -> Vec<u8> {
    instructions
        .iter()
        .flat_map(|instr| instr.encode())
        .collect()
}
