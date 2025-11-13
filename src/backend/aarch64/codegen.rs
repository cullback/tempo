use crate::backend::ir::{Instruction, Register};

fn register_name(reg: &Register) -> &'static str {
    match reg {
        Register::X0 => "x0",
        Register::X1 => "x1",
        Register::X2 => "x2",
        Register::X3 => "x3",
        Register::X4 => "x4",
        Register::X5 => "x5",
        Register::X6 => "x6",
        Register::X7 => "x7",
        Register::X8 => "x8",
    }
}

pub fn emit_instruction(instr: &Instruction) -> String {
    match instr {
        Instruction::MovImm { dest, value } => {
            format!("    mov {}, #{}", register_name(dest), value)
        }
        Instruction::AdrPcRel { dest, offset: _ } => {
            format!("    adr {}, .Ldata", register_name(dest))
        }
        Instruction::Syscall => "    svc #0".to_string(),
        Instruction::Add { dest, src1, src2 } => {
            format!(
                "    add {}, {}, {}",
                register_name(dest),
                register_name(src1),
                register_name(src2)
            )
        }
        Instruction::Sub { dest, src1, src2 } => {
            format!(
                "    sub {}, {}, {}",
                register_name(dest),
                register_name(src1),
                register_name(src2)
            )
        }
        Instruction::Mul { dest, src1, src2 } => {
            format!(
                "    mul {}, {}, {}",
                register_name(dest),
                register_name(src1),
                register_name(src2)
            )
        }
        Instruction::Div { dest, src1, src2 } => {
            format!(
                "    sdiv {}, {}, {}",
                register_name(dest),
                register_name(src1),
                register_name(src2)
            )
        }
        Instruction::Label { name } => format!("{}:", name),
        Instruction::Branch { condition, target } => {
            format!("    cbnz {}, {}", register_name(condition), target)
        }
        Instruction::Jump { target } => format!("    b {}", target),
        Instruction::Mov { dest, src } => {
            format!("    mov {}, {}", register_name(dest), register_name(src))
        }
    }
}
