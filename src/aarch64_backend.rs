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

    pub fn compile(program: &Program) -> Vec<u8> {
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

        binary
    }

    pub fn write_binary(
        binary: &[u8],
        output_path: &str,
    ) -> std::io::Result<()> {
        let mut file = File::create(output_path)?;
        file.write_all(binary)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ssa::{ModuleBuilder, STDOUT, SYS_EXIT, SYS_WRITE, Terminator};
    use crate::ssa_lowering;

    #[test]
    fn test_hello_world_binary_size() {
        let mut builder = ModuleBuilder::default();

        let block = builder.push_bb();
        builder.switch_to_block(block);

        let fd = builder.push_variable();
        builder.load_const(fd, STDOUT);

        let buf = builder.push_variable();
        builder.load_data_addr(buf, 0);

        let len = builder.push_variable();
        builder.load_const(len, 12);

        let write_syscall = builder.push_variable();
        builder.load_const(write_syscall, SYS_WRITE);

        builder.build_syscall(vec![write_syscall, fd, buf, len]);

        let exit_code = builder.push_variable();
        builder.load_const(exit_code, 0);

        let exit_syscall = builder.push_variable();
        builder.load_const(exit_syscall, SYS_EXIT);

        builder.build_syscall(vec![exit_syscall, exit_code]);

        builder.set_terminator(Terminator::ReturnVoid);
        builder.set_data(b"Hello World\n".to_vec());

        let module = builder.build_module();
        let ir_program = ssa_lowering::lower(&module);
        let binary = AArch64Backend::compile(&ir_program);
        assert_eq!(binary.len(), 164);
    }

    #[test]
    fn test_hello_world_from_ast() {
        use crate::ast::*;

        let source = "hello world";
        let span = Span::new(source);

        let program = Program {
            assignments: vec![
                Assignment {
                    identifier: Identifier {
                        name: "fd".to_string(),
                        span,
                    },
                    expression: Box::new(Expression::Number(Number {
                        value: 1,
                        span,
                    })),
                    span,
                },
                Assignment {
                    identifier: Identifier {
                        name: "msg".to_string(),
                        span,
                    },
                    expression: Box::new(Expression::FunctionCall(
                        FunctionCall {
                            function_name: Identifier {
                                name: "string_literal".to_string(),
                                span,
                            },
                            arguments: vec![Expression::Identifier(
                                Identifier {
                                    name: "Hello World\n".to_string(),
                                    span,
                                },
                            )],
                            span,
                        },
                    )),
                    span,
                },
                Assignment {
                    identifier: Identifier {
                        name: "len".to_string(),
                        span,
                    },
                    expression: Box::new(Expression::Number(Number {
                        value: 12,
                        span,
                    })),
                    span,
                },
                Assignment {
                    identifier: Identifier {
                        name: "_".to_string(),
                        span,
                    },
                    expression: Box::new(Expression::FunctionCall(
                        FunctionCall {
                            function_name: Identifier {
                                name: "write".to_string(),
                                span,
                            },
                            arguments: vec![
                                Expression::Identifier(Identifier {
                                    name: "fd".to_string(),
                                    span,
                                }),
                                Expression::Identifier(Identifier {
                                    name: "msg".to_string(),
                                    span,
                                }),
                                Expression::Identifier(Identifier {
                                    name: "len".to_string(),
                                    span,
                                }),
                            ],
                            span,
                        },
                    )),
                    span,
                },
                Assignment {
                    identifier: Identifier {
                        name: "_exit".to_string(),
                        span,
                    },
                    expression: Box::new(Expression::FunctionCall(
                        FunctionCall {
                            function_name: Identifier {
                                name: "exit".to_string(),
                                span,
                            },
                            arguments: vec![Expression::Number(Number {
                                value: 0,
                                span,
                            })],
                            span,
                        },
                    )),
                    span,
                },
            ],
            span,
        };

        println!("AST:\n{}", program);

        let lowering = AstLowering::new();
        let module = lowering.lower_program(&program);
        let ir_program = ssa_lowering::lower(&module);
        let binary = AArch64Backend::compile(&ir_program);
        assert_eq!(binary.len(), 164);
    }
}
