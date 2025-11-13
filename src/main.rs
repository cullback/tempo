mod aarch64_backend;
mod ast;
mod ir;
mod parser;
mod regalloc;
mod smol_hello;
mod ssa;
mod ssa_lowering;

use aarch64_backend::AArch64Backend;
use ast::AstLowering;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input.rb>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let source = fs::read_to_string(input_path).unwrap();

    let program = parser::parse_program(&source).unwrap();
    println!("AST:\n{}", program);

    let lowering = AstLowering::new();
    let module = lowering.lower_program(&program);

    println!("\nSSA IR:");
    for (i, block) in module.blocks.iter().enumerate() {
        println!("Block {}:", i);
        for instr in &block.instructions {
            println!("  {:?}", instr);
        }
    }

    let ir_program = ssa_lowering::lower(&module);

    println!("\nLowered IR:");
    for (i, instr) in ir_program.instructions.iter().enumerate() {
        println!("{:3}: {:?}", i, instr);
    }

    let binary = AArch64Backend::compile(&ir_program);

    let input_file = std::path::Path::new(input_path);
    let output_name = input_file.file_stem().unwrap().to_str().unwrap();
    AArch64Backend::write_binary(&binary, output_name).unwrap();
}
