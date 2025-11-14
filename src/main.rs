mod ast;
mod backend;
mod ssa;

use ast::{AstLowering, parse_program};
use backend::{assemble_and_link, compile};
use ssa::lower;
use std::env;
use std::fs;

fn hello() -> Vec<u8> {
    use backend::{Instruction, Register};

    let instructions = vec![
        Instruction::MovImm {
            rd: Register::X0,
            imm: 1,
        },
        Instruction::Adr {
            rd: Register::X1,
            offset: 28,
        },
        Instruction::MovImm {
            rd: Register::X2,
            imm: 13,
        },
        Instruction::MovImm {
            rd: Register::X8,
            imm: 64,
        },
        Instruction::Svc { imm: 0 },
        Instruction::MovImm {
            rd: Register::X0,
            imm: 0,
        },
        Instruction::MovImm {
            rd: Register::X8,
            imm: 93,
        },
        Instruction::Svc { imm: 0 },
    ];

    println!("Assembly:");
    for instr in &instructions {
        println!("  {}", instr);
    }

    let code = backend::encode_instructions(&instructions);
    let data = b"Hello World\n\0";

    backend::elf_bytes::generate_elf(&code, data)
}

fn main() {
    let binary = hello();

    fs::write("hello", &binary).expect("Failed to write binary");

    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata("hello").unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions("hello", perms).unwrap();

    println!("Created binary: hello");
}
