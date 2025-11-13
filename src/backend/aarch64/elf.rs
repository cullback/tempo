use crate::backend::ir::Program;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use super::codegen;

pub fn compile(program: &Program) -> String {
    let mut asm = String::new();

    asm.push_str(".global _start\n");
    asm.push_str(".text\n");
    asm.push_str("_start:\n");

    for instr in &program.instructions {
        asm.push_str(&codegen::emit_instruction(instr));
        asm.push('\n');
    }

    if !program.data.is_empty() {
        asm.push_str("\n.Ldata:\n");
        asm.push_str("    .byte ");
        for (i, byte) in program.data.iter().enumerate() {
            if i > 0 {
                asm.push_str(", ");
            }
            asm.push_str(&format!("{}", byte));
        }
        asm.push('\n');
    }

    asm
}

pub fn write_assembly(asm: &str, output_path: &str) -> std::io::Result<()> {
    let asm_path = format!("{}.s", output_path);
    let mut file = File::create(&asm_path)?;
    file.write_all(asm.as_bytes())?;
    println!("Created {}", asm_path);
    Ok(())
}

pub fn assemble_and_link(asm: &str, output_path: &str) -> std::io::Result<()> {
    let asm_path = format!("{}.s", output_path);
    let obj_path = format!("{}.o", output_path);

    let mut file = File::create(&asm_path)?;
    file.write_all(asm.as_bytes())?;

    let assemble_status = Command::new("as")
        .args(&["-o", &obj_path, &asm_path])
        .status()?;

    if !assemble_status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Assembly failed",
        ));
    }

    let link_status = Command::new("ld")
        .args(&["-o", output_path, &obj_path, "-s", "--nmagic"])
        .status()?;

    if !link_status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Linking failed",
        ));
    }

    std::fs::remove_file(&obj_path)?;

    println!("Created {} (from {})", output_path, asm_path);
    Ok(())
}
