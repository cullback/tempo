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

#[derive(Debug, Clone)]
pub enum Instruction {
    MovImm {
        rd: Register,
        imm: u16,
    },
    Adr {
        rd: Register,
        offset: i32,
    },
    Svc {
        imm: u16,
    },
    Add {
        rd: Register,
        rn: Register,
        rm: Register,
    },
    Sub {
        rd: Register,
        rn: Register,
        rm: Register,
    },
    Mul {
        rd: Register,
        rn: Register,
        rm: Register,
    },
    Div {
        rd: Register,
        rn: Register,
        rm: Register,
    },
    Mov {
        rd: Register,
        rm: Register,
    },
    Cmp {
        rn: Register,
        rm: Register,
    },
    CSet {
        rd: Register,
        cond: Condition,
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
    Call {
        target: String,
    },
    Ret,
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
            Instruction::Add { rd, rn, rm } => {
                // ADD rd, rn, rm
                let instr = 0x8b000000u32
                    | (rm.num() << 16)
                    | (rn.num() << 5)
                    | rd.num();
                instr.to_le_bytes().to_vec()
            }
            Instruction::Sub { rd, rn, rm } => {
                // SUB rd, rn, rm
                let instr = 0xcb000000u32
                    | (rm.num() << 16)
                    | (rn.num() << 5)
                    | rd.num();
                instr.to_le_bytes().to_vec()
            }
            Instruction::Mul { rd, rn, rm } => {
                // MUL rd, rn, rm
                let instr = 0x9b007c00u32
                    | (rm.num() << 16)
                    | (rn.num() << 5)
                    | rd.num();
                instr.to_le_bytes().to_vec()
            }
            Instruction::Div { rd, rn, rm } => {
                // SDIV rd, rn, rm
                let instr = 0x9ac00c00u32
                    | (rm.num() << 16)
                    | (rn.num() << 5)
                    | rd.num();
                instr.to_le_bytes().to_vec()
            }
            Instruction::Mov { rd, rm } => {
                // MOV rd, rm (encoded as ORR rd, xzr, rm)
                let instr = 0xaa0003e0u32 | (rm.num() << 16) | rd.num();
                instr.to_le_bytes().to_vec()
            }
            Instruction::Cmp { rn, rm } => {
                // CMP rn, rm (encoded as SUBS xzr, rn, rm)
                let instr = 0xeb00001fu32 | (rm.num() << 16) | (rn.num() << 5);
                instr.to_le_bytes().to_vec()
            }
            Instruction::CSet { rd, cond } => {
                // CSET rd, cond
                let cond_code = match cond {
                    Condition::Eq => 0,
                    Condition::Ne => 1,
                    Condition::Lt => 11,
                    Condition::Le => 13,
                    Condition::Gt => 12,
                    Condition::Ge => 10,
                };
                let instr = 0x9a9f07e0u32 | ((cond_code ^ 1) << 12) | rd.num();
                instr.to_le_bytes().to_vec()
            }
            Instruction::Ret => {
                // RET
                let instr = 0xd65f03c0u32;
                instr.to_le_bytes().to_vec()
            }
            Instruction::Label { .. }
            | Instruction::Branch { .. }
            | Instruction::Jump { .. }
            | Instruction::Call { .. } => {
                // These need label resolution, return empty for now
                vec![]
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
            Instruction::Add { rd, rn, rm } => {
                write!(f, "add {}, {}, {}", rd, rn, rm)
            }
            Instruction::Sub { rd, rn, rm } => {
                write!(f, "sub {}, {}, {}", rd, rn, rm)
            }
            Instruction::Mul { rd, rn, rm } => {
                write!(f, "mul {}, {}, {}", rd, rn, rm)
            }
            Instruction::Div { rd, rn, rm } => {
                write!(f, "sdiv {}, {}, {}", rd, rn, rm)
            }
            Instruction::Mov { rd, rm } => {
                write!(f, "mov {}, {}", rd, rm)
            }
            Instruction::Cmp { rn, rm } => {
                write!(f, "cmp {}, {}", rn, rm)
            }
            Instruction::CSet { rd, cond } => {
                let cond_str = match cond {
                    Condition::Eq => "eq",
                    Condition::Ne => "ne",
                    Condition::Lt => "lt",
                    Condition::Le => "le",
                    Condition::Gt => "gt",
                    Condition::Ge => "ge",
                };
                write!(f, "cset {}, {}", rd, cond_str)
            }
            Instruction::Label { name } => {
                write!(f, "{}:", name)
            }
            Instruction::Branch { condition, target } => {
                write!(f, "cbnz {}, {}", condition, target)
            }
            Instruction::Jump { target } => {
                write!(f, "b {}", target)
            }
            Instruction::Call { target } => {
                write!(f, "bl {}", target)
            }
            Instruction::Ret => write!(f, "ret"),
        }
    }
}

pub fn encode_instructions(instructions: &[Instruction]) -> Vec<u8> {
    use std::collections::HashMap;

    // First pass: calculate label positions
    let mut label_positions: HashMap<String, usize> = HashMap::new();
    let mut pc = 0;

    for instr in instructions {
        match instr {
            Instruction::Label { name } => {
                label_positions.insert(name.clone(), pc);
            }
            Instruction::Branch { .. }
            | Instruction::Jump { .. }
            | Instruction::Call { .. } => {
                pc += 4; // These are 4 bytes each
            }
            _ => {
                pc += instr.encode().len();
            }
        }
    }

    // Second pass: encode with resolved labels
    let mut code = Vec::new();
    let mut current_pc = 0;

    for instr in instructions {
        match instr {
            Instruction::Label { .. } => {
                // Labels don't emit bytes
            }
            Instruction::Branch { condition, target } => {
                let target_pc =
                    label_positions.get(target).expect("Undefined label");
                let offset = (*target_pc as i32 - current_pc as i32) / 4;

                // CBNZ condition, offset
                let encoded = 0x35000000u32
                    | ((offset as u32 & 0x7ffff) << 5)
                    | condition.num();
                code.extend_from_slice(&encoded.to_le_bytes());
                current_pc += 4;
            }
            Instruction::Jump { target } => {
                let target_pc =
                    label_positions.get(target).expect("Undefined label");
                let offset = (*target_pc as i32 - current_pc as i32) / 4;

                // B offset
                let encoded = 0x14000000u32 | (offset as u32 & 0x3ffffff);
                code.extend_from_slice(&encoded.to_le_bytes());
                current_pc += 4;
            }
            Instruction::Call { target } => {
                let target_pc =
                    label_positions.get(target).expect("Undefined label");
                let offset = (*target_pc as i32 - current_pc as i32) / 4;

                // BL offset
                let encoded = 0x94000000u32 | (offset as u32 & 0x3ffffff);
                code.extend_from_slice(&encoded.to_le_bytes());
                current_pc += 4;
            }
            _ => {
                let bytes = instr.encode();
                current_pc += bytes.len();
                code.extend_from_slice(&bytes);
            }
        }
    }

    code
}

fn convert_register(reg: &crate::backend::ir::Register) -> Register {
    match reg {
        crate::backend::ir::Register::X0 => Register::X0,
        crate::backend::ir::Register::X1 => Register::X1,
        crate::backend::ir::Register::X2 => Register::X2,
        crate::backend::ir::Register::X3 => Register::X3,
        crate::backend::ir::Register::X4 => Register::X4,
        crate::backend::ir::Register::X5 => Register::X5,
        crate::backend::ir::Register::X6 => Register::X6,
        crate::backend::ir::Register::X7 => Register::X7,
        crate::backend::ir::Register::X8 => Register::X8,
    }
}

fn convert_condition(cond: &crate::backend::ir::Condition) -> Condition {
    match cond {
        crate::backend::ir::Condition::Eq => Condition::Eq,
        crate::backend::ir::Condition::Ne => Condition::Ne,
        crate::backend::ir::Condition::Lt => Condition::Lt,
        crate::backend::ir::Condition::Le => Condition::Le,
        crate::backend::ir::Condition::Gt => Condition::Gt,
        crate::backend::ir::Condition::Ge => Condition::Ge,
    }
}

pub fn from_ir(ir_instr: &crate::backend::ir::Instruction) -> Instruction {
    match ir_instr {
        crate::backend::ir::Instruction::MovImm { dest, value } => {
            Instruction::MovImm {
                rd: convert_register(dest),
                imm: *value as u16,
            }
        }
        crate::backend::ir::Instruction::AdrPcRel { dest, offset } => {
            Instruction::Adr {
                rd: convert_register(dest),
                offset: *offset,
            }
        }
        crate::backend::ir::Instruction::Syscall => Instruction::Svc { imm: 0 },
        crate::backend::ir::Instruction::Add { dest, src1, src2 } => {
            Instruction::Add {
                rd: convert_register(dest),
                rn: convert_register(src1),
                rm: convert_register(src2),
            }
        }
        crate::backend::ir::Instruction::Sub { dest, src1, src2 } => {
            Instruction::Sub {
                rd: convert_register(dest),
                rn: convert_register(src1),
                rm: convert_register(src2),
            }
        }
        crate::backend::ir::Instruction::Mul { dest, src1, src2 } => {
            Instruction::Mul {
                rd: convert_register(dest),
                rn: convert_register(src1),
                rm: convert_register(src2),
            }
        }
        crate::backend::ir::Instruction::Div { dest, src1, src2 } => {
            Instruction::Div {
                rd: convert_register(dest),
                rn: convert_register(src1),
                rm: convert_register(src2),
            }
        }
        crate::backend::ir::Instruction::Mov { dest, src } => {
            Instruction::Mov {
                rd: convert_register(dest),
                rm: convert_register(src),
            }
        }
        crate::backend::ir::Instruction::Cmp { src1, src2 } => {
            Instruction::Cmp {
                rn: convert_register(src1),
                rm: convert_register(src2),
            }
        }
        crate::backend::ir::Instruction::CSet { dest, condition } => {
            Instruction::CSet {
                rd: convert_register(dest),
                cond: convert_condition(condition),
            }
        }
        crate::backend::ir::Instruction::Label { name } => {
            Instruction::Label { name: name.clone() }
        }
        crate::backend::ir::Instruction::Branch { condition, target } => {
            Instruction::Branch {
                condition: convert_register(condition),
                target: target.clone(),
            }
        }
        crate::backend::ir::Instruction::Jump { target } => Instruction::Jump {
            target: target.clone(),
        },
        crate::backend::ir::Instruction::Call { target } => Instruction::Call {
            target: target.clone(),
        },
        crate::backend::ir::Instruction::Ret => Instruction::Ret,
    }
}

pub fn generate_elf_from_ir(program: &crate::backend::ir::Program) -> Vec<u8> {
    let mut instructions: Vec<Instruction> =
        program.instructions.iter().map(from_ir).collect();

    println!("Assembly:");
    for instr in &instructions {
        println!("  {}", instr);
    }

    // Calculate where data will be: after code section
    let mut code_size = 0;
    for instr in &instructions {
        match instr {
            Instruction::Label { .. } => {} // Labels don't emit bytes
            Instruction::Branch { .. }
            | Instruction::Jump { .. }
            | Instruction::Call { .. } => {
                code_size += 4;
            }
            _ => {
                code_size += instr.encode().len();
            }
        }
    }

    // Base addresses from elf_bytes
    let code_offset = 0x78u64;
    let code_vaddr = 0x400078u64;
    let data_vaddr = code_vaddr + code_size as u64;

    // Fix ADR instructions to point to the actual data location
    let mut pc = code_vaddr;
    for instr in &mut instructions {
        match instr {
            Instruction::Label { .. } => {} // Labels don't advance PC
            Instruction::Adr { rd, offset } => {
                // Recalculate offset from current PC to data
                let new_offset = (data_vaddr as i64 - pc as i64) as i32;
                *offset = new_offset;
                pc += 4;
            }
            Instruction::Branch { .. }
            | Instruction::Jump { .. }
            | Instruction::Call { .. } => {
                pc += 4;
            }
            _ => {
                pc += instr.encode().len() as u64;
            }
        }
    }

    let code = encode_instructions(&instructions);
    crate::backend::elf_bytes::generate_elf(&code, &program.data)
}
