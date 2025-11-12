use crate::ir::{Instruction, Program, Register};
use std::fs::File;
use std::io::Write;

pub struct AArch64Backend;

impl AArch64Backend {
    fn encode_register(reg: &Register) -> u8 {
        match reg {
            Register::X0 => 0,
            Register::X1 => 1,
            Register::X2 => 2,
            Register::X8 => 8,
        }
    }

    fn encode_mov_imm(dest: &Register, value: u64) -> [u8; 4] {
        let rd = Self::encode_register(dest);
        let imm16 = (value & 0xffff) as u16;
        let hw = 0;

        let encoding = 0xd2800000u32
            | ((hw & 0b11) << 21)
            | ((imm16 as u32) << 5)
            | (rd as u32);

        encoding.to_le_bytes()
    }

    fn encode_adr(dest: &Register, offset: i32) -> [u8; 4] {
        let rd = Self::encode_register(dest);
        let immlo = (offset & 0b11) as u32;
        let immhi = ((offset >> 2) & 0x7ffff) as u32;

        let encoding =
            0x10000000u32 | (immlo << 29) | (immhi << 5) | (rd as u32);

        encoding.to_le_bytes()
    }

    fn encode_syscall() -> [u8; 4] {
        [0x01, 0x00, 0x00, 0xd4]
    }

    fn encode_instruction(instr: &Instruction) -> Vec<u8> {
        match instr {
            Instruction::MovImm { dest, value } => {
                Self::encode_mov_imm(dest, *value).to_vec()
            }
            Instruction::AdrPcRel { dest, offset } => {
                Self::encode_adr(dest, *offset).to_vec()
            }
            Instruction::Syscall => Self::encode_syscall().to_vec(),
        }
    }

    pub fn compile(
        program: &Program,
        output_path: &str,
    ) -> std::io::Result<()> {
        let mut binary = Vec::new();

        let code_size = program.instructions.len() * 4;
        let data_offset = 0x78 + code_size;
        let total_size = data_offset + program.data.len();

        binary.extend_from_slice(&[
            0x7f, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
            0xb7, 0x00, 1, 0, 0, 0, 0x78, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
            0x00, 0x38, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);

        binary.extend_from_slice(&[
            0x01, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);

        binary.push(total_size as u8);
        binary.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        binary.push(total_size as u8);
        binary.extend_from_slice(&[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);

        for instr in &program.instructions {
            binary.extend_from_slice(&Self::encode_instruction(instr));
        }

        binary.extend_from_slice(&program.data);

        let mut file = File::create(output_path)?;
        file.write_all(&binary)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(output_path, perms)?;
        }

        println!("Created {} ({} bytes)", output_path, binary.len());
        Ok(())
    }
}
