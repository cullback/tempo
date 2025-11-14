pub mod aarch64;
pub mod elf_bytes;
pub mod ir;
pub mod regalloc;

pub use aarch64::{
    Instruction, Register, assemble_and_link, assemble_and_link_to_bytes,
    compile, encode_instructions,
};
pub use elf_bytes::generate_hello_elf;
