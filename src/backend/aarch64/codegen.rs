use crate::backend::ir::{Instruction, Register};

pub fn encode_register(reg: &Register) -> u8 {
    match reg {
        Register::X0 => 0,
        Register::X1 => 1,
        Register::X2 => 2,
        Register::X3 => 3,
        Register::X4 => 4,
        Register::X5 => 5,
        Register::X6 => 6,
        Register::X7 => 7,
        Register::X8 => 8,
    }
}

pub fn encode_mov_imm(dest: &Register, value: u64) -> [u8; 4] {
    let rd = encode_register(dest);
    let imm16 = (value & 0xffff) as u16;
    let hw = 0;

    let encoding = 0xd2800000u32
        | ((hw & 0b11) << 21)
        | ((imm16 as u32) << 5)
        | (rd as u32);

    encoding.to_le_bytes()
}

pub fn encode_adr(dest: &Register, offset: i32) -> [u8; 4] {
    let rd = encode_register(dest);
    let immlo = (offset & 0b11) as u32;
    let immhi = ((offset >> 2) & 0x7ffff) as u32;

    let encoding = 0x10000000u32 | (immlo << 29) | (immhi << 5) | (rd as u32);

    encoding.to_le_bytes()
}

pub fn encode_syscall() -> [u8; 4] {
    [0x01, 0x00, 0x00, 0xd4]
}

pub fn encode_add(
    dest: &Register,
    src1: &Register,
    src2: &Register,
) -> [u8; 4] {
    let rd = encode_register(dest);
    let rn = encode_register(src1);
    let rm = encode_register(src2);

    let encoding =
        0x8b000000u32 | ((rm as u32) << 16) | ((rn as u32) << 5) | (rd as u32);
    encoding.to_le_bytes()
}

pub fn encode_sub(
    dest: &Register,
    src1: &Register,
    src2: &Register,
) -> [u8; 4] {
    let rd = encode_register(dest);
    let rn = encode_register(src1);
    let rm = encode_register(src2);

    let encoding =
        0xcb000000u32 | ((rm as u32) << 16) | ((rn as u32) << 5) | (rd as u32);
    encoding.to_le_bytes()
}

pub fn encode_mul(
    dest: &Register,
    src1: &Register,
    src2: &Register,
) -> [u8; 4] {
    let rd = encode_register(dest);
    let rn = encode_register(src1);
    let rm = encode_register(src2);

    let encoding =
        0x9b007c00u32 | ((rm as u32) << 16) | ((rn as u32) << 5) | (rd as u32);
    encoding.to_le_bytes()
}

pub fn encode_div(
    dest: &Register,
    src1: &Register,
    src2: &Register,
) -> [u8; 4] {
    let rd = encode_register(dest);
    let rn = encode_register(src1);
    let rm = encode_register(src2);

    let encoding =
        0x9ac00c00u32 | ((rm as u32) << 16) | ((rn as u32) << 5) | (rd as u32);
    encoding.to_le_bytes()
}

pub fn encode_instruction(instr: &Instruction) -> Vec<u8> {
    match instr {
        Instruction::MovImm { dest, value } => {
            encode_mov_imm(dest, *value).to_vec()
        }
        Instruction::AdrPcRel { dest, offset } => {
            encode_adr(dest, *offset).to_vec()
        }
        Instruction::Syscall => encode_syscall().to_vec(),
        Instruction::Add { dest, src1, src2 } => {
            encode_add(dest, src1, src2).to_vec()
        }
        Instruction::Sub { dest, src1, src2 } => {
            encode_sub(dest, src1, src2).to_vec()
        }
        Instruction::Mul { dest, src1, src2 } => {
            encode_mul(dest, src1, src2).to_vec()
        }
        Instruction::Div { dest, src1, src2 } => {
            encode_div(dest, src1, src2).to_vec()
        }
    }
}
