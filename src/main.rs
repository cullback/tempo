mod ast;
mod backend;
mod ssa;

use ast::{AstLowering, parse_program};
use backend::{assemble_and_link, compile};
use ssa::lower;
use std::env;
use std::fs;

fn hello() -> Vec<u8> {
    backend::generate_hello_elf()
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
