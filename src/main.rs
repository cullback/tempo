mod ast;
mod backend;
mod ssa;

use ast::{AstLowering, parse_program};
use backend::generate_elf_from_ir;
use ssa::lower;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <input.rb> [output]", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = if args.len() == 3 {
        args[2].clone()
    } else {
        std::path::Path::new(input_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output")
            .to_string()
    };

    let source = fs::read_to_string(input_path).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", input_path, e);
        std::process::exit(1);
    });

    let program = parse_program(&source).unwrap_or_else(|e| {
        eprintln!("Failed to parse: {}", e);
        std::process::exit(1);
    });

    println!("Program AST:");
    println!("{}", program);
    println!();

    let lowering = AstLowering::new();
    let module = lowering.lower_program(&program);

    println!("SSA:");
    println!("{:#?}", module);
    println!();

    let ir_program = lower(&module);

    println!("IR:");
    for instr in &ir_program.instructions {
        println!("  {:?}", instr);
    }
    println!();

    let binary = generate_elf_from_ir(&ir_program);

    fs::write(&output_path, &binary).unwrap_or_else(|e| {
        eprintln!("Failed to write {}: {}", output_path, e);
        std::process::exit(1);
    });

    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&output_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&output_path, perms).unwrap();

    println!("Created binary: {}", output_path);
}
