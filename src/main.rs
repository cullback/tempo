mod ast;
mod backend;
mod ssa;

use ast::{AstLowering, parse_program};
use backend::{assemble_and_link, compile};
use ssa::lower;
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

    let program = parse_program(&source).unwrap();
    println!("AST:\n{}", program);

    let lowering = AstLowering::new();
    let module = lowering.lower_program(&program);

    println!("\nModule data: {} bytes", module.data.len());

    println!("\nSSA IR:");
    for (i, block) in module.blocks.iter().enumerate() {
        println!("Block {} (params: {:?}):", i, block.params);
        for instr in &block.instructions {
            println!("  {:?}", instr);
        }
        println!("  -> {}", block.terminator);
    }

    println!("\nFunctions:");
    for (name, func) in &module.functions {
        println!("Function '{}' (params: {:?}):", name, func.params);
        for (i, block) in func.blocks.iter().enumerate() {
            println!("  Block {} (params: {:?}):", i, block.params);
            for instr in &block.instructions {
                println!("    {:?}", instr);
            }
            println!("    -> {}", block.terminator);
        }
    }

    let ir_program = lower(&module);

    println!("\nLowered IR:");
    for (i, instr) in ir_program.instructions.iter().enumerate() {
        println!("{:3}: {:?}", i, instr);
    }
    println!(
        "\nData section ({} bytes): {:?}",
        ir_program.data.len(),
        ir_program.data
    );

    let asm = compile(&ir_program);

    println!("\nAssembly:\n{}", asm);

    let input_file = std::path::Path::new(input_path);
    let output_name = input_file.file_stem().unwrap().to_str().unwrap();
    assemble_and_link(&asm, output_name).unwrap();
}
